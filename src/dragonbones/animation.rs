use shipyard::prelude::*;
use shipyard_scenegraph::*;
use nalgebra::{Unit, UnitQuaternion, Vector3};
use wasm_bindgen::UnwrapThrowExt;
use std::collections::{HashMap, BTreeMap};
use std::collections::VecDeque;
use crate::components::*;
use crate::config::*;
use crate::textures::data::Texture;
use super::data::*;
use super::bones::{BoneToEntity, SlotToBone};
use super::skin::BoneEntityToTextureEntity;

/*
    This doesn't create a direct 1:1 mapping of dragonbones
    Rather, the focus is on generating straightforward, re-usable timelines

    Ultimately, the TweenTimelines it creates are like this, one for each animation name:

    AnimationTimeline(
        AnimationGroup[
            TranslationSequence[
                TranslationGroup1[
                    TranslationSequenceForBone1[..],
                    TranslationSequenceForBone2[..],
                    TranslationSequenceForBoneN[..],
                ],
                TranslationGroup2[
                    TranslationSequenceForBone1[..],
                    TranslationSequenceForBone2[..],
                    TranslationSequenceForBoneN[..],
                ],
            ],
            RotationSequence[
                RotationGroup1[
                    RotationSequenceForBone1[..],
                    RotationSequenceForBone2[..],
                    RotationSequenceForBoneN[..],
                ],
                RotationGroup2[
                    RotationSequenceForBone1[..],
                    RotationSequenceForBone2[..],
                    RotationSequenceForBoneN[..],
                ],
            ],
            ColorSequence[
                ColorGroup1[
                    ColorSequenceForBone1[..],
                    ColorSequenceForBone2[..],
                    ColorSequenceForBoneN[..],
                ],
                ColorGroup2[
                    ColorSequenceForBone1[..],
                    ColorSequenceForBone2[..],
                    ColorSequenceForBoneN[..],
                ],
            ],
        ]
    )

    Effectively this means that:
    1. The entire animation plays as a group (will cycle when last tween finishes)
    2. Each top-level sequence plays its grouped tweens together (all bones at a time) until the last one finishes
*/

//Given an animation name, get a TweenTimeline
type AnimationToTimeline = HashMap<String, TweenTimeline>;

//internal use only - to append to the group at a sequence index
fn add_to_group(lookup:&mut BTreeMap<usize, Vec<Tween>>, seq_index:usize, value:Tween) {
    lookup
        .entry(seq_index)
        .or_default()
        .push(value);
}

fn into_sequence_of_groups(lookup:BTreeMap<usize, Vec<Tween>>) -> Option<TweenTimeline> {
    let seq:Vec<TweenTimeline> = 
        lookup
            .into_iter()
            .map(|(key, tweens)| tweens)
            .filter(|tweens| tweens.len() != 0)
            .map(|tweens| tweens.into_iter().map(|tween| TweenTimeline::Clip(tween)))
            .map(|tweens| TweenTimeline::Group(Box::new(tweens.collect())))
            .collect();

    if seq.len() == 0 {
        None
    } else {
        Some(TweenTimeline::Sequence(Box::new(seq)))
    }
}

pub fn create_animations(world:&World, root:EntityId, armature:&Armature, bone_to_entity:&BoneToEntity, slot_to_bone:&SlotToBone, bone_to_texture:&BoneEntityToTextureEntity, tex_height: f64) {
    let tweens_lookup = create_animations_lookup(world, armature, bone_to_entity, slot_to_bone, bone_to_texture, tex_height);
    let (entities, mut tweens_lookup_storage) = world.borrow::<(EntitiesMut, &mut TweensLookup)>();
    entities.add_component(&mut tweens_lookup_storage, TweensLookup(tweens_lookup), root);
}
pub fn create_animations_lookup(world:&World, armature:&Armature, bone_to_entity:&BoneToEntity, slot_to_bone:&SlotToBone, bone_to_texture:&BoneEntityToTextureEntity, tex_height: f64) -> AnimationToTimeline {
    let mut tweens_lookup:AnimationToTimeline = HashMap::new();

    let mut last_bone_translation = HashMap::<EntityId, Vec3>::new();
    let mut last_bone_rotation = HashMap::<EntityId, f64>::new();
    let mut last_bone_scale = HashMap::<EntityId, Vec3>::new();

    {
        world.run::<(&Translation, &Rotation, &Scale), _, _>(|(translations, rotations, scales)| {
            bone_to_entity.iter().for_each(|(name, entity)| {
                let tex_entity = bone_to_texture.get(entity).unwrap_or(entity);
                let (t,r,s) = (&translations, &rotations, &scales).get(*entity).unwrap_throw();
                last_bone_translation.insert(*entity, t.0);
                //TODO - get original rotation as degree angle 
                last_bone_rotation.insert(*tex_entity, 0.0);
                //last_bone_rotation.insert(*entity, r.0);
                last_bone_scale.insert(*entity, s.0);
            });
        })
    }

    struct AnimationGroup {
        translations: BTreeMap<usize, Vec<Tween>>,
        rotations: BTreeMap<usize, Vec<Tween>>,
        scales: BTreeMap<usize, Vec<Tween>>,
        colors: BTreeMap<usize, Vec<Tween>>,
    }

    for anim in armature.animations.iter() {

        let anim_name = anim.name.to_string();

        let mut animation_group = AnimationGroup {
            translations: BTreeMap::new(),
            rotations: BTreeMap::new(),
            scales: BTreeMap::new(),
            colors: BTreeMap::new(),
        };
      
        //TODO - in order to rotate around the center
        //Might need to add *additional* keyframes to move the child or parent
        //The basic setup is already there with entity vs. tex_entity
        //However we may need to also get tex width/height
        if let Some(anim_bones) = anim.bones.as_ref() {
            
            for (bone_index, anim_bone) in anim_bones.iter().enumerate() {
                let entity = bone_to_entity.get(&anim_bone.name).unwrap_throw();

                let tex_entity = bone_to_texture.get(entity).unwrap_or(entity);

                if let Some(anim_translations) = &anim_bone.translation_frames {
                    for (seq_index, anim_translation) in anim_translations.iter().enumerate() {
                        let duration = anim_translation.duration.unwrap_or(1.0) * DRAGONBONES_BASE_SPEED;
                        let easing = anim_translation.easing.and_then(|easing| if easing == 0.0 { None } else { Some(easing) });


                        let last = last_bone_translation.get_mut(entity).unwrap_throw();

                        let anim_x = anim_translation.x.map(|x| last.x + x);
                        let anim_y = anim_translation.y.map(|y| last.y - y);

                        add_to_group(&mut animation_group.translations, seq_index, 
                            Tween::Translation(
                                Vec3Tween {
                                    info: TweenInfo {
                                        entity: *entity,
                                        easing,
                                        duration,
                                    },
                                    x: anim_x.map(|x| (last.x, x)),
                                    y: anim_y.map(|y| (last.y, y)),
                                    z: None,
                                }
                            )
                        );

                        if let Some(x) = anim_x {
                            last.x = x;
                        }
                        if let Some(y) = anim_y {
                            last.y = y;
                        }
                    }
                }
                
                if let Some(anim_rotations) = &anim_bone.rotation_frames {
                    for (seq_index, anim_rotation) in anim_rotations.iter().enumerate() {
                        let duration = anim_rotation.duration.unwrap_or(1.0) * DRAGONBONES_BASE_SPEED;
                        let easing = anim_rotation.easing.and_then(|easing| if easing == 0.0 { None } else { Some(easing) });

                        let last = last_bone_rotation.get_mut(tex_entity).unwrap_throw();
                        let anim_rot = anim_rotation.rotation.map(|rot| *last + rot);
                        add_to_group(&mut animation_group.rotations, seq_index, 
                            Tween::Rotation(
                                ScalarTween {
                                    info: TweenInfo {
                                        entity: *tex_entity,
                                        easing,
                                        duration,
                                    },
                                    value: anim_rot.map(|rot| (*last, rot)),
                                }
                            )
                        );
                        if let Some(rot) = anim_rot {
                            *last = rot;
                        }
                    }
                }
            }
        }

        if let Some(anim_slots) = anim.slots.as_ref() {
            for anim_slot in anim_slots.iter() {
                if let Some(anim_colors) = &anim_slot.color_frames {
                    let bone_name = slot_to_bone.get(&anim_slot.slot_name).unwrap_throw();
                    let entity = bone_to_entity.get(bone_name).unwrap_throw();

                    for (seq_index, anim_color) in anim_colors.iter().enumerate() {
                        let duration = anim_color.duration.unwrap_or(1.0) * DRAGONBONES_BASE_SPEED;
                        let easing = anim_color.easing.and_then(|easing| if easing == 0.0 { None } else { Some(easing) });
                        let value = &anim_color.value;
                        add_to_group(&mut animation_group.colors, seq_index, 
                            Tween::ColorAdjust(
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
                        ));
                    }
                }
            }
        }

        let mut groups_of_sequences:Vec<TweenTimeline> = Vec::new(); 

        if let Some(seq) = into_sequence_of_groups(animation_group.translations) {
            groups_of_sequences.push(seq);
        }
        if let Some(seq) = into_sequence_of_groups(animation_group.rotations) {
            groups_of_sequences.push(seq);
        }
        if let Some(seq) = into_sequence_of_groups(animation_group.scales) {
            groups_of_sequences.push(seq);
        }
        if let Some(seq) = into_sequence_of_groups(animation_group.colors) {
            groups_of_sequences.push(seq);
        }

        
        tweens_lookup.insert(anim.name.to_string(), TweenTimeline::Group(Box::new(groups_of_sequences)));
    }
   
    tweens_lookup
}