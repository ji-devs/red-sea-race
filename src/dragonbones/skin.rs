use shipyard::prelude::*;
use shipyard_scenegraph::spawn_child;
use wasm_bindgen::prelude::*;
use std::collections::HashMap;
use crate::textures::data::Texture;
use crate::components::Renderable;
use super::data::{Armature};
use super::bones::{get_bone_trs, BoneToEntity};

pub type BoneEntityToTextureEntity = HashMap<EntityId, EntityId>;

pub fn create_skin_entities(world:&World, bone_to_entity:&BoneToEntity, textures:&HashMap<String, Texture>, armature:&Armature, atlas_height: f64) -> BoneEntityToTextureEntity {
    let mut bone_to_texture:BoneEntityToTextureEntity = HashMap::new();

    for (parent, entity, texture) in armature.skins
        .iter()
        .flat_map(|skin| skin.slots.iter())
        .flat_map(|slot| {
            let parent = bone_to_entity.get(&slot.bone_name).unwrap_throw();

            if slot.display.iter().count() > 1 {
                panic!("TODO - support multiple textures somehow");
            }

            slot.display
                .iter()
                .map(move |display| {
                    let texture = textures.get(&display.texture_name).unwrap_throw().clone();
                    let (mut translation, rotation, scale) = get_bone_trs(&display.transform, atlas_height);
                    
                    //TODO - use anchor point instead
                    translation.x -= (texture.tex_width as f64)/2.0;
                    translation.y -= (texture.tex_height as f64)/2.0;

                    let entity = spawn_child(world, Some(*parent), Some(translation), None, Some(scale));

                    (*parent, entity, texture)
                })
        }) {
            bone_to_texture.insert(parent, entity);
            let renderable = Renderable { texture, flip: false};
            let (entities, mut renderable_storage) = world.borrow::<(EntitiesMut, &mut Renderable)>();
            entities.add_component(&mut renderable_storage, renderable, entity);
        }

    bone_to_texture
}
