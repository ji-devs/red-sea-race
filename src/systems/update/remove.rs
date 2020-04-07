use shipyard::prelude::*;
use shipyard_scenegraph::{Translation, Parent, Child, HierarchyMut};
use crate::components::*;

#[system(TrashSys)]
pub fn run(
    mut all_storages: &mut AllStorages,
) {
    
    let entities_to_delete:Vec<EntityId> = {
        let (translations, sprites, renderables) = all_storages.borrow::<(&Translation, &Sprite, &Renderable)>();

        let entities_to_delete:Vec<EntityId> = 
            (&translations, &sprites, &renderables)
                .iter()
                .with_id()
                .filter(|(_, (pos, _layer, renderable))| {
                    let right_bound = pos.x + (renderable.texture.tex_width as f64); 

                    //less than 0.0 is probably fine, but let's give it a bit of padding to be safe
                    if right_bound < 2.0 { true } else { false }
                })
                .map(|(entity, _)| entity)
                .collect();
       
        
        let mut hierarchy_storages  = all_storages.borrow::<(EntitiesMut, &mut Parent, &mut Child)>();

        let mut hierarchy_storages = (&mut hierarchy_storages.0, &mut hierarchy_storages.1, &mut hierarchy_storages.2);
        for entity in entities_to_delete.iter() {
            hierarchy_storages.remove_single(*entity);
        }

        entities_to_delete
    };

    for entity in entities_to_delete {
        all_storages.delete(entity);
    }
    
}