use shipyard::prelude::*;
use shipyard_scenegraph::*;
use nalgebra::Vector3;
use wasm_bindgen::UnwrapThrowExt;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use crate::components::*;
use crate::media::*;
use crate::config::*;
use crate::texture::*;

pub fn create_dragonbones_entity(world:&World, dragonbones:&DragonBones) -> EntityId {
    
    let (root_entity, bone_entity_lookup) = create_bone_entities(world, &dragonbones);
    let sprites:Vec<(EntityId, Texture)> = {

        bone_entity_lookup.iter().flat_map(|(bone_name, entity)| {
            //log::info!("{} {:?}", bone_name, entity);
            create_bone_textures(world, &dragonbones, *entity, bone_name)
        })
        .collect()
    };

    let (entities, mut renderables) = world.borrow::<(EntitiesMut, &mut Renderable)>();
    sprites.into_iter().for_each(|(entity, texture)| {
        entities.add_component(&mut renderables, Renderable { texture, flip: false}, entity);
    });

    root_entity

}

type Trs = (Option<Vec3>, Option<Quat>, Option<Vec3>);

fn create_bone_textures(world:&World, character:&DragonBones, parent:EntityId, bone_name:&str) -> Vec<(EntityId, Texture)>{
    let textures = &character.textures;
    let skeleton = &character.skeleton;
    let armature = &skeleton.armatures[0];
    let parent = Some(parent);

    armature.skins
        .iter()
        .flat_map(|skin| skin.slots.iter())
        .filter(|slot| slot.bone_name == bone_name)
        .flat_map(|slot| {
            slot.display.iter().map(|display| {
                let texture = textures.get(&display.texture_name).unwrap_throw().clone();
                let (translation, rotation, scale) = get_bone_trs(&display.transform, 0.0);
                let mut translation = translation.unwrap_or(Vector3::new(0.0, 0.0, 0.0));
                
                //TODO - use anchor point instead
                translation.x -= (texture.tex_width as f64)/2.0;
                translation.y -= (texture.tex_height as f64)/2.0;

                let entity = spawn_child(world, parent, Some(translation), rotation, scale);

                (entity, texture)
            })
        })
        .collect()
}

fn create_bone_entities(world:&World, character:&DragonBones) -> (EntityId, HashMap<String, EntityId>) {

    let root_entity = spawn_child(world, None, None, None, None);
    let skeleton = &character.skeleton;

    let armature = &skeleton.armatures[0];

    /* Since the bone listing might be in any order
        create the tree in multiple stages
    */
   
    let mut bone_entity_lookup:HashMap<String, EntityId> = HashMap::new();
    let mut bone_children_lookup:HashMap<String, Vec<(String, Trs)>> = HashMap::new();
    let mut parents:VecDeque<String> = VecDeque::new();

    armature.bones.iter().enumerate().for_each(|(layer_index, bone)| {
        match &bone.parent_name {
            //if its a root element, create the entity and add it as a parent
            None => {
                let trs = get_bone_trs(&bone.transform, CHARACTER_SPRITE_DEPTH);
                let entity = spawn_child(world, Some(root_entity), trs.0, trs.1, trs.2);
                bone_entity_lookup.insert(bone.name.to_string(), entity);
                if !parents.contains(&bone.name) {
                    parents.push_back(bone.name.to_string());
                }
            },
            //otherwise, stash the child for processing later 
            Some(parent_name) => {
                log::info!("{}", bone.name.to_string());
                //TODO - is layer_index *really* the right determination of layering?
                //Seems broken... use top-level `slot` instead?
                let trs = get_bone_trs(&bone.transform, layer_index as f64);
                if !bone_children_lookup.contains_key(parent_name) {
                    bone_children_lookup.insert(parent_name.to_string(), Vec::new());
                }

                bone_children_lookup.get_mut(parent_name).unwrap_throw().push(
                    (bone.name.to_string(), trs)
                );
            }
        };
    });


    //now iterate over all parents and add the children proper (with their trs)
    //make sure to only iterate over parents after their entity has been created
    //(there's no guarantee that the order matches the tree insertion order)
    loop {
        match parents.pop_front() {
            None => break,
            Some(parent_name) => {
                let parent_entity = {
                    *bone_entity_lookup.get(&parent_name).unwrap_throw()
                };
                if let Some(children) = bone_children_lookup.get(&parent_name) {
                    for (bone_name, trs) in children {
                        let entity = spawn_child(world, Some(parent_entity), trs.0, trs.1, trs.2);
                        bone_entity_lookup.insert(bone_name.to_string(), entity);
                        if !parents.contains(&bone_name) {
                            parents.push_back(bone_name.to_string());
                        }
                    }
                }
            }
        }
    }

    (root_entity, bone_entity_lookup)
}
fn get_bone_trs(transform:&Option<BoneTransform>, layer_depth: f64) -> Trs { 
    match transform {
        None => (
            Some(Vector3::new(0.0, 0.0, layer_depth)), None, None
        ),
        Some(transform) => {
            let x = transform.x.unwrap_or(0.0);
            let y = 512.0 - transform.y.unwrap_or(0.0);
            let skew_x = transform.skew_x.unwrap_or(0.0);
            let skew_y = transform.skew_y.unwrap_or(0.0);
            let scale_x = transform.scale_x.unwrap_or(1.0);
            let scale_y = transform.scale_y.unwrap_or(1.0);

            (
                Some(Vector3::new(x, y, layer_depth)), 
                None, 
                Some(Vector3::new(scale_x, scale_y, 1.0))
            )
        } 
    }
}