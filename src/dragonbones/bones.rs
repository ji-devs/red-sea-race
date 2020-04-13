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

/* So far just supporting 1 armature */

//TODO - core tween tests are passing - so see if we're really getting the right structure
//It's possible that the bones aren't a sequence or something?

//Given a bone name, get its entity and a Vec of its children texture-entities
//type BoneMap = HashMap<String, (EntityId, Vec<EntityId>)>;
// 
// 

//Given a bone name, get its entity
pub type BoneToEntity = HashMap<String, EntityId>;
//Given a slot name, get the bone name
pub type SlotToBone = HashMap<String, String>;

//create the root dragonbones entity as well as its children, per-bone
//each entity will have only the scenegraph components (parent, child, transforms, etc.)
//the texture height needs to be passed in for flipping the y-coordinates
//returns root entity and a lookup of bone name -> entity
pub fn create_bone_entities(world:&World, root_entity:EntityId, armature:&Armature, tex_height: f64) -> BoneToEntity {

    let mut bone_entity_lookup = HashMap::new();

    /* 
        Since the bone listing might be in any order
        create the hierarchy in different stages
        for example we can first set the depth lookup table
        but scenegraph hierarchy comes later 
        (since top-level roots can be listed _after_ their children)
    */
 
    //given a bone by name, get its depth (i.e. layer, z-coordinate)
    let mut bone_depth_lookup:HashMap<&str, f64> = HashMap::new();
    for (depth, slot) in armature.slots.iter().enumerate() {
        bone_depth_lookup.insert(&slot.bone, depth as f64);
    }

    //given a bone by name, get its children bones as (name, trs) pairs
    let mut bone_children_lookup:HashMap<&str, Vec<(String, Trs)>> = HashMap::new();
    let mut parents:VecDeque<&str> = VecDeque::new();

    armature.bones.iter().for_each(|bone| {
        match &bone.parent_name {
            //if its a root element, create the entity and add it as a parent
            None => {
                let (mut translation, rotation, scale) = get_bone_trs(&bone.transform, tex_height);
                translation.z = CHARACTER_SPRITE_DEPTH;
                let entity = spawn_child(world, Some(root_entity), Some(translation), None, Some(scale));
                bone_entity_lookup.insert(bone.name.to_string(), entity);
                if !parents.contains(&bone.name.as_ref()) {
                    parents.push_back(&bone.name);
                }
            },
            //otherwise, stash the child for processing later 
            Some(parent_name) => {
                let mut trs = get_bone_trs(&bone.transform, tex_height);
                if let Some(depth) = bone_depth_lookup.get(&bone.name.as_ref()) {
                    trs.0.z = *depth;
                }
                bone_children_lookup
                    .entry(parent_name)
                    .or_insert(Vec::new())
                    .push((bone.name.to_string(), trs));
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
                    for (bone_name, (translation, rotation, scale)) in children {
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
    
    bone_entity_lookup
}
pub fn create_slot_lookup(armature:&Armature) -> SlotToBone {
    let mut slot_to_bone:SlotToBone = HashMap::new();

    for slot in armature.slots.iter() {
        slot_to_bone.insert(slot.name.to_string(), slot.bone.to_string());
    }

    slot_to_bone
}



pub type Trs = (Vec3, Quat, Vec3);
pub fn get_bone_trs(transform:&Option<BoneTransform>, tex_height: f64) -> Trs { 
    let mut translation:Vec3 = Vector3::new(0.0, 0.0, 0.0);
    let mut rotation:Quat= Quat::identity();
    let mut scale:Vec3 = Vector3::new(1.0, 1.0, 1.0);

    //let tex_height:f64 = 512.0;

    if let Some(transform) = transform {
        let x = transform.x.unwrap_or(0.0);
        let y = tex_height - transform.y.unwrap_or(0.0);
        //TODO - does skew affect trs?
        let _skew_x = transform.skew_x.unwrap_or(0.0);
        let _skew_y = transform.skew_y.unwrap_or(0.0);
        let scale_x = transform.scale_x.unwrap_or(1.0);
        let scale_y = transform.scale_y.unwrap_or(1.0);

        translation.x = x;
        translation.y = y;

        scale.x = scale_x;
        scale.y = scale_y;
    }

    (translation, rotation, scale)
}

/*
pub fn create_entity(world:&World, dragonbones:&DragonBones) -> EntityId {
  
    //Need to do all this in a block
    //Since we'll borrow EntitiesMut to add the components later
    let (root_entity, renderables, tweens_lookup) = {

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

        let tweens_lookup = create_tweens_lookup(dragonbones, &bonemap, &slot_to_bone);

        (root_entity, renderables, tweens_lookup)
    };

     
    //Can't borrow EntitiesMut while doing the above
    let (entities, mut renderable_storage, mut tweens_lookup_storage) = world.borrow::<(EntitiesMut, &mut Renderable, &mut TweensLookup)>();
    for (entity, renderable) in renderables {
        entities.add_component(&mut renderable_storage, renderable, entity);
    }
    entities.add_component(&mut tweens_lookup_storage, tweens_lookup, root_entity);
    root_entity

}

fn create_tweens_lookup(dragonbones:&DragonBones, bonemap:&BoneMap, slot_to_bone:&SlotToBone) -> TweensLookup {
    let mut tweens_lookup:HashMap<String, TweenTimeline> = HashMap::new();

    let skeleton = &dragonbones.skeleton;
    let armature = &skeleton.armatures[0];

    for anim in armature.animations.iter() {

        let bone_groups:Option<Vec<TweenTimeline>> = anim.bones.as_ref().map(|anim_bones| {
            anim_bones.iter().map(|anim_bone| {
                let mut tweens = Vec::new();

                let (entity, _) = bonemap.get(&anim_bone.name).unwrap_throw();

                if let Some(anim_translations) = &anim_bone.translation_frames {
                    for anim_translation in anim_translations {
                        let duration = anim_translation.duration.unwrap_or(1.0) * 1000.0;
                        let easing = anim_translation.easing.and_then(|easing| if easing == 0.0 { None } else { Some(easing) });

                        if anim.name == "run" {
                            log::info!("initial add... {:?} {} {:?} {:?}", entity, anim_bone.name, anim_translation.x, anim_translation.y);
                        }
                        tweens.push(TweenTimeline::Clip(Tween::Translation(
                            Vec3Tween {
                                info: TweenInfo {
                                    entity: *entity,
                                    easing,
                                    duration,
                                },
                                //TODO get real start value
                                x: anim_translation.x.map(|x| (0.0, x)),
                                y: anim_translation.y.map(|y| (0.0, y)),
                                z: None,
                            }
                        )));
                    }
                }
                
                if let Some(anim_rotations) = &anim_bone.rotation_frames {
                    for anim_rotation in anim_rotations {
                        let duration = anim_rotation.duration.unwrap_or(1.0) * 1000.0;
                        let easing = anim_rotation.easing.and_then(|easing| if easing == 0.0 { None } else { Some(easing) });
                        let (x, y, z, w) = match anim_rotation.rotation {
                            None => (None, None, None, None),
                            //TODO - get rotation start and end quat
                            Some(rotation) => (None, None, None, None)
                        };
                        tweens.push(TweenTimeline::Clip(Tween::Rotation(
                            QuatTween {
                                info: TweenInfo {
                                    entity: *entity,
                                    easing,
                                    duration,
                                },
                                x,
                                y,
                                z,
                                w
                            }
                        )));
                    }
                }
                TweenTimeline::Group(Box::new(tweens))
            })
            .collect()
        });

        let slot_groups:Option<Vec<TweenTimeline>> = anim.slots.as_ref().map(|anim_slots| {
            anim_slots.iter().map(|anim_slot| {
                let mut tweens = Vec::new();
                if let Some(anim_colors) = &anim_slot.color_frames {
                    let bone_name = slot_to_bone.get(&anim_slot.slot_name).unwrap_throw();
                    let (_, textures) = bonemap.get(bone_name).unwrap_throw();

                    for anim_color in anim_colors {
                        let duration = anim_color.duration.unwrap_or(1.0) * 1000.0;
                        let easing = anim_color.easing.and_then(|easing| if easing == 0.0 { None } else { Some(easing) });
                        let value = &anim_color.value;
                        for entity in textures {
                            tweens.push(TweenTimeline::Clip(Tween::ColorAdjust(
                                ColorTween {
                                    info: TweenInfo {
                                        entity: *entity,
                                        easing,
                                        duration,
                                    },
                                    //TODO - use start value from this point in time 
                                    alpha_overlay: value.alpha_overlay.map(|c| (0.0, c)),
                                    red_overlay: value.red_overlay.map(|c| (0.0, c)),
                                    green_overlay: value.green_overlay.map(|c| (0.0, c)),
                                    blue_overlay: value.blue_overlay.map(|c| (0.0, c)),
                                    
                                    alpha_offset: value.alpha_offset.map(|c| (0.0, c)),
                                    red_offset: value.red_offset.map(|c| (0.0, c)),
                                    green_offset: value.green_offset.map(|c| (0.0, c)),
                                    blue_offset: value.blue_offset.map(|c| (0.0, c)),
                                }
                            )));
                        }
                    }
                }
                TweenTimeline::Group(Box::new(tweens))
            })
            .collect()
        });
      
        let mut sequences:Vec<TweenTimeline> = Vec::new(); 
        if let Some(bone_groups) = bone_groups {
            sequences.push(TweenTimeline::Sequence(Box::new(bone_groups)));
        }
        if let Some(slot_groups) = slot_groups {
            sequences.push(TweenTimeline::Sequence(Box::new(slot_groups)));
        }
        tweens_lookup.insert(anim.name.to_string(), TweenTimeline::Group(Box::new(sequences)));
    }
   
    TweensLookup (tweens_lookup)
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

*/