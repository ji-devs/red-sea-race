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

/*
    High level goals:
    1. The entire animation plays as a group (may cycle when last tween finishes)
    2. Each top-level sequence plays its grouped tweens together (all bones at a time) until the last one finishes
    3. The tweens don't need to know about dragonbones nuance (like inverted y)
    4. More specifically, tween architecture shouldn't even be affected by dragonbones opinions

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

fn into_sequence_of_groups(mut lookup:BTreeMap<usize, Vec<Tween>>) -> Option<TweenTimeline> {
    let mut seq:Vec<TweenTimeline> = 
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

//These two are separated to make unit testing easier (i.e. to test the lookup without having to grab it from the world)
pub fn create_animations(world:&World, root:EntityId, armature:&Armature, bone_to_entity:&BoneToEntity, slot_to_bone:&SlotToBone, tex_height: f64) {
    let tweens_lookup = create_animations_lookup(world, armature, bone_to_entity, slot_to_bone, tex_height);
    let (entities, mut tweens_lookup_storage) = world.borrow::<(EntitiesMut, &mut TweensLookup)>();
    entities.add_component(&mut tweens_lookup_storage, TweensLookup(tweens_lookup), root);
}

/*
    Overall, dragonbones describes its animations as delta changes - and leaves out untweened properties
    This makes sense to keep on-disk data lightweight but needs work to turn it into usable runtime data
    Also, the nesting structure in dragonbones makes it hard to reason about from a "play this animation"
    perspective. In other words we want a sequence of frames, not frames of sequences.
*/

pub fn create_animations_lookup(world:&World, armature:&Armature, bone_to_entity:&BoneToEntity, slot_to_bone:&SlotToBone, atlas_height: f64) -> AnimationToTimeline {
    let mut tweens_lookup:AnimationToTimeline = HashMap::new();

    let mut first_bone_translation = HashMap::<EntityId, Vec3>::new();
    let mut first_bone_rotation = HashMap::<EntityId, f64>::new();
    let mut first_bone_scale = HashMap::<EntityId, Vec3>::new();

    {
        world.run::<(&Translation, &Rotation, &Scale), _, _>(|(translations, rotations, scales)| {
            bone_to_entity.iter().for_each(|(name, entity)| {
                let (t,r,s) = (&translations, &rotations, &scales).get(*entity).unwrap_throw();
                let t = t.0;
                let r = 0.0; //TODO - get original rotation as degree angle, shouldn't assume it starts at 0!
                let s = s.0;
                first_bone_translation.insert(*entity, t);
                first_bone_rotation.insert(*entity, 0.0);
                first_bone_scale.insert(*entity, s);
            });
        })
    }

    //In order to make it easier to invert the frames of sequences -> sequences of frames
    //we need to stash each frame at its sequence index, then when its all done we can iterate
    //the inner Vec<Tween> here is not a sequence - rather, it's a group that plays all at once (e.g. bones-per-frame)
    struct AnimationGroup {
        translations: BTreeMap<usize, Vec<Tween>>,
        rotations: BTreeMap<usize, Vec<Tween>>,
        scales: BTreeMap<usize, Vec<Tween>>,
        colors: BTreeMap<usize, Vec<Tween>>,
    }

    //each sequence
    for anim in armature.animations.iter() {

        let mut first_tween_translation = HashMap::<EntityId, TweenData<Vec3>>::new();
        let mut first_tween_rotation = HashMap::<EntityId, TweenData<Quat>>::new();
        let mut first_tween_scale = HashMap::<EntityId, TweenData<Vec3>>::new();

        let anim_name = anim.name.to_string();
        let mut last_bone_translation = HashMap::<EntityId, Vec3>::new();
        let mut last_bone_rotation = HashMap::<EntityId, f64>::new();
        let mut last_bone_scale = HashMap::<EntityId, Vec3>::new();
        bone_to_entity.iter().for_each(|(name, entity)| {
            last_bone_translation.insert(*entity, first_bone_translation.get(entity).unwrap_throw().clone());
            last_bone_rotation.insert(*entity, first_bone_rotation.get(entity).unwrap_throw().clone());
            last_bone_scale.insert(*entity, first_bone_scale.get(entity).unwrap_throw().clone());
        });
        
        let mut animation_group = AnimationGroup {
            translations: BTreeMap::new(),
            rotations: BTreeMap::new(),
            scales: BTreeMap::new(),
            colors: BTreeMap::new(),
        };
      
        if let Some(anim_bones) = anim.bones.as_ref() {
            
            for (bone_index, anim_bone) in anim_bones.iter().enumerate() {
                let bone_name = &anim_bone.name;
                let entity = *bone_to_entity.get(&anim_bone.name).unwrap_throw();
                let first_translation = first_bone_translation.get(&entity).unwrap_throw();
                let first_rotation = first_bone_rotation.get(&entity).unwrap_throw();


                if let Some(anim_translations) = &anim_bone.translation_frames {
                    let mut duration = 0.0; 
                    for (seq_index, anim_translation) in anim_translations.iter().enumerate() {
                        let mut easing = anim_translation.easing.and_then(|easing| if easing == 0.0 { None } else { Some(easing) });


                        let last = last_bone_translation.get_mut(&entity).unwrap_throw();

                        let next_x = anim_translation.x.map(|x| last.x + x).unwrap_or(last.x);
                        let next_y = anim_translation.y.map(|y| last.y - y).unwrap_or(last.y);

                        let tween = Tween::new_translation(
                            Vec3::new(last.x, last.y, first_translation.z),
                            Vec3::new(next_x, next_y, first_translation.z),
                            duration,
                            Some(entity),
                            None
                        ); 

                        if seq_index == 0 {
                            first_tween_translation.insert(entity, tween.get_translation_data().unwrap_throw().clone());
                        }
                        
                      
                        //loop by making last frame a mixture of first and last
                        if seq_index == anim_translations.len()-1 {
                            let mut loop_data = first_tween_translation.get(&entity).unwrap_throw().clone();
                            loop_data.from.x = next_x;
                            loop_data.from.y = next_y;
                            loop_data.to.x = first_translation.x;
                            loop_data.to.y = first_translation.y;
                            loop_data.info.duration = duration;
                            add_to_group(&mut animation_group.translations, seq_index, Tween::Translation(loop_data));
                        } else {
                            add_to_group(&mut animation_group.translations, seq_index, tween);
                        }

                        last.x = next_x;
                        last.y = next_y;
                        
                        duration = anim_translation.duration.unwrap_or(1.0) * DRAGONBONES_BASE_SPEED;

                    }
                }
                
                if let Some(anim_rotations) = &anim_bone.rotation_frames {

                    let mut duration = 0.0; 
                    for (seq_index, anim_rotation) in anim_rotations.iter().enumerate() {
                        let mut easing = anim_rotation.easing.and_then(|easing| if easing == 0.0 { None } else { Some(easing) });

                        let last = last_bone_rotation.get_mut(&entity).unwrap_throw();
                        let next = anim_rotation.rotation.map(|rot| *last + rot).unwrap_or(*last);
                        
                        let tween = Tween::new_rotation(
                            rotation_to_quat(*last),
                            rotation_to_quat(next),
                            duration,
                            Some(entity),
                            None
                        ); 

                        if seq_index == 0 {
                            first_tween_rotation.insert(entity, tween.get_rotation_data().unwrap_throw().clone());
                        }
                        

                        //loop by making last frame a mixture of first and last
                        if seq_index == anim_rotations.len()-1 {
                            let mut loop_data = first_tween_rotation.get(&entity).unwrap_throw().clone();
                            loop_data.from = rotation_to_quat(next);
                            loop_data.to= rotation_to_quat(*first_rotation);
                            loop_data.info.duration = duration;
                            add_to_group(&mut animation_group.rotations, seq_index, Tween::Rotation(loop_data));
                        } else {
                            add_to_group(&mut animation_group.rotations, seq_index, tween);
                        }

                        *last = next;
                        
                        duration = anim_rotation.duration.unwrap_or(1.0) * DRAGONBONES_BASE_SPEED;
                    }
                }
            }
        }

        //TODO - color tweens have not really been added yet 
        if let Some(anim_slots) = anim.slots.as_ref() {
            for anim_slot in anim_slots.iter() {
                if let Some(anim_colors) = &anim_slot.color_frames {
                    let bone_name = slot_to_bone.get(&anim_slot.slot_name).unwrap_throw();
                    let entity = bone_to_entity.get(bone_name).unwrap_throw();

                    for (seq_index, anim_color) in anim_colors.iter().enumerate() {
                        let duration = anim_color.duration.unwrap_or(1.0) * DRAGONBONES_BASE_SPEED;
                        let easing = anim_color.easing.and_then(|easing| if easing == 0.0 { None } else { Some(easing) });
                        //TODO
                        //let last = last_bone_color.get_mut(&entity).unwrap_throw();
                        //let next = anim_color.color.map(|color| *last + color).unwrap_or(*last);
                        //add_to_group(&mut animation_group.colors, seq_index, Tween::ColorAdjust(...))
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

fn rotation_to_quat(rot:f64) -> UnitQuaternion<f64> {
    let axis = Unit::new_normalize(Vector3::new(0.0, 0.0, -1.0));
    UnitQuaternion::from_axis_angle(&axis, rot.to_radians())
}