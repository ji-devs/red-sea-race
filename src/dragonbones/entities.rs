use shipyard::prelude::*;
use shipyard_scenegraph::*;
use nalgebra::Vector3;
use wasm_bindgen::UnwrapThrowExt;
use std::collections::HashMap;
use std::collections::VecDeque;
use crate::components::*;
use crate::config::*;
use crate::textures::data::Texture;
use super::data::*;

//Given a bone name, get its entity and a Vec of its children texture-entities
type BoneMap = HashMap<String, (EntityId, Vec<EntityId>)>;
//Given a slot name, get the bone name
type SlotToBone = HashMap<String, String>;

//create the root dragonbones entity as well as its children
//all will have the required components added too:
//* scenegraph (parent, child, transforms, etc.)
//* renderable
//* animator
pub fn create_entity(world:&World, dragonbones:&DragonBones) -> EntityId {
  
    //Need to do all this in a block
    //Since we'll borrow EntitiesMut to add the components later
    let (root_entity, renderables, animation_sequences) = {

        //Need to create all the bones and textures
        //As well as preserve a lookup table for animations

        //First get the root entity and bone lookups (will have empty children here)
        let slot_to_bone = create_slot_lookup(&dragonbones);
        let (root_entity, mut bonemap) = create_bone_entities(world, &dragonbones);

        //Then get the renderables that are associated with the bone's texture children
        //At the same time, populate those children in the lookup
        let renderables:Vec<(EntityId, Renderable)> = 
            bonemap
                .iter_mut()
                .flat_map(|(bone_name, (bone_entity, ref mut bone_texture_entities))| {
                    //get all the textured children
                    let textures = create_bone_textures(world, &dragonbones, *bone_entity, bone_name);
                    //associate the entity with this bone in the lookup
                    for (entity, _) in &textures {
                        bone_texture_entities.push(*entity);
                    }
                    
                    //get a renderable from the texture and entity
                    textures
                        .into_iter()
                        .map(|(entity, texture)| {
                            (entity, Renderable { texture, flip: false})
                        })
                })
                .collect();

        let animator = create_animator(dragonbones, &bonemap, &slot_to_bone);

        (root_entity, renderables, animator)
    };

     
    //Can't borrow EntitiesMut while doing the above
    let (entities, mut renderable_storage, mut animation_sequences_storage) = world.borrow::<(EntitiesMut, &mut Renderable, &mut AnimationSequences)>();
    for (entity, renderable) in renderables {
        entities.add_component(&mut renderable_storage, renderable, entity);
    }
    entities.add_component(&mut animation_sequences_storage, animation_sequences, root_entity);
    root_entity

}

fn create_animator(dragonbones:&DragonBones, bonemap:&BoneMap, slot_to_bone:&SlotToBone) -> AnimationSequences {
    let mut sequences:HashMap<String, AnimationSequence> = HashMap::new();

    let skeleton = &dragonbones.skeleton;
    let armature = &skeleton.armatures[0];

    for anim in armature.animations.iter() {
        //TODO - accumulate real total duration!
        let mut total_duration = 2.0 * 1000.0;
        let mut animations:Vec<Animation> = Vec::new();

        if let Some(anim_bones) = anim.bones.as_ref() {
            for anim_bone in anim_bones.iter() {
                let (entity, _) = bonemap.get(&anim_bone.name).unwrap_throw();

                if let Some(anim_translations) = &anim_bone.translation_frames {
                    for anim_translation in anim_translations {
                        let duration = anim_translation.duration.unwrap_or(1.0) * 1000.0;
                        let easing = anim_translation.easing.and_then(|easing| if easing == 0.0 { None } else { Some(easing) });
                        animations.push(Animation {
                            entity: *entity,
                            easing,
                            duration,
                            target: AnimationTarget::Translation(TranslationAnimationTarget{
                                x: anim_translation.x,
                                y: anim_translation.y,
                            })
                        });
                    }
                }
                
                if let Some(anim_rotations) = &anim_bone.rotation_frames {
                    for anim_rotation in anim_rotations {
                        let duration = anim_rotation.duration.unwrap_or(1.0) * 1000.0;
                        let easing = anim_rotation.easing.and_then(|easing| if easing == 0.0 { None } else { Some(easing) });
                        animations.push(Animation {
                            entity: *entity,
                            easing,
                            duration,
                            target: AnimationTarget::Rotation(RotationAnimationTarget {
                                rotation: anim_rotation.rotation,
                            })
                        });
                    }
                }
            }
        }
        if let Some(anim_slots) = anim.slots.as_ref() {
            for anim_slot in anim_slots.iter() {
                if let Some(anim_colors) = &anim_slot.color_frames {
                    let bone_name = slot_to_bone.get(&anim_slot.slot_name).unwrap_throw();
                    let (_, textures) = bonemap.get(bone_name).unwrap_throw();

                    for anim_color in anim_colors {
                        let duration = anim_color.duration.unwrap_or(1.0) * 1000.0;
                        let easing = anim_color.easing.and_then(|easing| if easing == 0.0 { None } else { Some(easing) });
                        let value = &anim_color.value;
                        for entity in textures {
                            animations.push(Animation {
                                entity: *entity,
                                easing,
                                duration,
                                target: AnimationTarget::Color(ColorAnimationTarget {
                                    alpha_overlay: value.alpha_overlay,
                                    red_overlay: value.red_overlay,
                                    green_overlay: value.green_overlay,
                                    blue_overlay: value.blue_overlay,
                                    alpha_offset: value.alpha_offset,
                                    red_offset: value.red_offset,
                                    green_offset: value.green_offset,
                                    blue_offset: value.blue_offset,
                                })
                            });
                        }
                    }
                }
            }
        }
        sequences.insert(anim.name.to_string(), AnimationSequence { 
            animations,
            total_duration
        });
    }
   
    AnimationSequences( sequences)
}


fn create_slot_lookup(dragonbones:&DragonBones) -> SlotToBone {
    let mut slot_to_bone:SlotToBone = HashMap::new();

    let skeleton = &dragonbones.skeleton;
    let armature = &skeleton.armatures[0];

    for slot in armature.slots.iter() {
        slot_to_bone.insert(slot.name.to_string(), slot.bone.to_string());
    }

    slot_to_bone
}

fn create_bone_entities(world:&World, dragonbones:&DragonBones) -> (EntityId, BoneMap) {

    let mut bonemap:BoneMap = HashMap::new();

    let root_entity = spawn_child(world, None, None, None, None);

    let skeleton = &dragonbones.skeleton;

    let armature = &skeleton.armatures[0];

    /* Since the bone listing might be in any order
        create the tree in multiple stages
    */
  
    let mut bone_depth_lookup:HashMap<&str, f64> = HashMap::new();

    //Not 100% sure this is right, but seems okay!
    for (depth, slot) in armature.slots.iter().enumerate() {
        bone_depth_lookup.insert(&slot.bone, depth as f64);
    }

    let mut bone_children_lookup:HashMap<&str, Vec<(String, TranslationScale)>> = HashMap::new();
    let mut parents:VecDeque<&str> = VecDeque::new();

    armature.bones.iter().for_each(|bone| {
        match &bone.parent_name {
            //if its a root element, create the entity and add it as a parent
            None => {
                let (mut translation, scale) = get_bone_ts(&bone.transform);
                translation.z = CHARACTER_SPRITE_DEPTH;
                let entity = spawn_child(world, Some(root_entity), Some(translation), None, Some(scale));
                bonemap.insert(bone.name.to_string(), (entity, Vec::new()));
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
                    bonemap.get(parent_name).unwrap_throw().0
                };
                if let Some(children) = bone_children_lookup.get(&parent_name) {
                    for (bone_name, (translation, scale)) in children {
                        let entity = spawn_child(world, Some(parent_entity), Some(*translation), None, Some(*scale));
                        bonemap.insert(bone_name.to_string(), (entity, Vec::new()));
                        if !parents.contains(&bone_name.as_ref()) {
                            parents.push_back(&bone_name);
                        }
                    }
                }
            }
        }
    }

    (root_entity, bonemap)
}

fn create_bone_textures(world:&World, character:&DragonBones, parent:EntityId, bone_name:&str) -> Vec<(EntityId, Texture)> {
    let textures = &character.textures;
    let skeleton = &character.skeleton;
    let armature = &skeleton.armatures[0];
    let parent = Some(parent);

    armature.skins
        .iter()
        .flat_map(|skin| skin.slots.iter())
        .filter(|slot| slot.bone_name == bone_name)
        .flat_map(|slot| {
            slot.display
                .iter()
                .map(|display| {
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