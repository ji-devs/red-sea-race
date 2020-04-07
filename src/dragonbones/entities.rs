use shipyard::prelude::*;
use shipyard_scenegraph::*;
use nalgebra::Vector3;
use wasm_bindgen::UnwrapThrowExt;
use std::collections::HashMap;
use std::collections::VecDeque;
use crate::components::*;
use crate::media::*;
use crate::config::*;
use crate::textures::Texture;

pub fn create_entity(world:&World, dragonbones:&DragonBones) -> EntityId {
    
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
                let (mut translation, scale) = get_bone_ts(&display.transform);
                
                //TODO - use anchor point instead
                translation.x -= (texture.tex_width as f64)/2.0;
                translation.y -= (texture.tex_height as f64)/2.0;

                let entity = spawn_child(world, parent, Some(translation), None, Some(scale));

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
  
    let mut bone_depth_lookup:HashMap<&str, f64> = HashMap::new();

    //Not 100% sure this is right, but seems okay!
    for (depth, slot) in armature.slots.iter().enumerate() {
        bone_depth_lookup.insert(&slot.bone, depth as f64);
    }

    let mut bone_entity_lookup:HashMap<String, EntityId> = HashMap::new();
    let mut bone_children_lookup:HashMap<&str, Vec<(String, TranslationScale)>> = HashMap::new();
    let mut parents:VecDeque<&str> = VecDeque::new();

    armature.bones.iter().for_each(|bone| {
        match &bone.parent_name {
            //if its a root element, create the entity and add it as a parent
            None => {
                let (mut translation, scale) = get_bone_ts(&bone.transform);
                translation.z = CHARACTER_SPRITE_DEPTH;
                let entity = spawn_child(world, Some(root_entity), Some(translation), None, Some(scale));
                bone_entity_lookup.insert(bone.name.to_string(), entity);
                if !parents.contains(&bone.name.as_ref()) {
                    parents.push_back(&bone.name);
                }
            },
            //otherwise, stash the child for processing later 
            Some(parent_name) => {
                let mut ts = get_bone_ts(&bone.transform);
                if let Some(depth) = bone_depth_lookup.get(&bone.name.as_ref()) {
                    ts.0.z = *depth;
                }
                bone_children_lookup
                    .entry(parent_name)
                    .or_insert(Vec::new())
                    .push((bone.name.to_string(), ts));
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
                    *bone_entity_lookup.get(parent_name).unwrap_throw()
                };
                if let Some(children) = bone_children_lookup.get(&parent_name) {
                    for (bone_name, (translation, scale)) in children {
                        let entity = spawn_child(world, Some(parent_entity), Some(*translation), None, Some(*scale));
                        bone_entity_lookup.insert(bone_name.to_string(), entity);
                        if !parents.contains(&bone_name.as_ref()) {
                            parents.push_back(&bone_name);
                        }
                    }
                }
            }
        }
    }

    (root_entity, bone_entity_lookup)
}

//todo - require height
type TranslationScale = (Vec3, Vec3);
fn get_bone_ts(transform:&Option<BoneTransform>) -> TranslationScale { 
    let mut translation:Vec3 = Vector3::new(0.0, 0.0, 0.0);
    let mut scale:Vec3 = Vector3::new(1.0, 1.0, 1.0);
    let tex_height:f64 = 512.0;

    if let Some(transform) = transform {
        let x = transform.x.unwrap_or(0.0);
        let y = tex_height - transform.y.unwrap_or(0.0);
        let _skew_x = transform.skew_x.unwrap_or(0.0);
        let _skew_y = transform.skew_y.unwrap_or(0.0);
        let scale_x = transform.scale_x.unwrap_or(1.0);
        let scale_y = transform.scale_y.unwrap_or(1.0);

        translation.x = x;
        translation.y = y;

        scale.x = scale_x;
        scale.y = scale_y;
    }

    (translation, scale)
}