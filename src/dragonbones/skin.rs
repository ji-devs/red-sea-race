use shipyard::prelude::*;
use shipyard_scenegraph::{spawn_child, Origin, Translation};
use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use nalgebra::Vector3;
use crate::textures::data::Texture;
use crate::components::Renderable;
use super::data::{Armature};
use super::bones::{get_bone_trs, BoneToEntity};

/* Dragonbones skins actually cause the offset and origin to be moved to *graphical* center
    So if we have a skin - we update the bone's origin and initial translation
    Animation is described as delta changes so it'll just propogate from the initial setting
*/

pub fn create_skin_entities(world:&World, bone_to_entity:&BoneToEntity, textures:&HashMap<String, Texture>, armature:&Armature, atlas_height: f64) {
    for (entity, texture, origin) in armature.skins
        .iter()
        .flat_map(|skin| skin.slots.iter())
        .flat_map(|slot| {
            let entity = bone_to_entity.get(&slot.bone_name).unwrap_throw();

            if slot.display.iter().count() > 1 {
                panic!("TODO - support multiple textures somehow");
            }

            slot.display
                .iter()
                .map(move |display| {
                    let texture = textures.get(&display.texture_name).unwrap_throw().clone();
                    let (mut translation, rotation, scale) = get_bone_trs(&display.transform, atlas_height);
                  
                    let origin = Vector3::new((texture.tex_width as f64)/2.0, (texture.tex_height as f64)/2.0, 0.0);

                    (*entity, texture, origin)
                })
        }) {
            let renderable = Renderable { texture, flip: false};
            let origin = Origin(origin);
            let (entities, mut renderable_storage, mut origin_storage, mut translation_storage) = world.borrow::<(EntitiesMut, &mut Renderable, &mut Origin, &mut Translation)>();
            let translation = translation_storage[entity].0;
            let translation = Translation(Vector3::new(translation.x - origin.x, translation.y - origin.y, translation.z));
            entities.add_component((&mut renderable_storage, &mut origin_storage, &mut translation_storage), (renderable, origin, translation), entity);

        }
}
