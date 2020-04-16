use std::fs::File;
use std::path::Path;
use shipyard::prelude::*;
use shipyard_scenegraph as sg;
use super::bones::{create_bone_entities, create_slot_lookup};
use super::animation::{create_animations_lookup};
use super::data::Skeleton;
use crate::components::*;

#[test]
fn dragonbones() {
    let world = World::default();
    sg::init(&world);

    let json_path = Path::new("./_static/media/images/characters/israelite_ske.json");
    let file = File::open(json_path).unwrap();
    let skeleton:Skeleton = serde_json::from_reader(file).unwrap();
    let armature = &skeleton.armatures[0];

    let root = sg::spawn_child(&world, None, None, None, None, None);
    let bone_to_entity = create_bone_entities(&world, root, &armature, 512.0);
    let slot_to_bone = create_slot_lookup(&armature);
    let animation_to_tween = create_animations_lookup(&world, &armature, &bone_to_entity, &slot_to_bone, 512.0);

    /*
        run some tests on the "run" animation
    */
    let timeline = animation_to_tween.get("run").expect("should have run animation");
    assert_eq!(timeline.duration(), 8.0 * 1000.0);
    let groups = timeline.as_group().expect("timeline should have groups");
    assert_eq!(groups.len(), 2); //translation and rotation
    for (timeline_group_index, timeline_group) in groups.iter().enumerate() {
        let sequence = timeline_group.as_sequence().expect("timeline should have sequence");
        assert_eq!(sequence.len(), 3); //translation and rotation both have 3 items 

        for (seq_group_index, seq_group) in sequence.iter().enumerate() {
            let seq_group = seq_group.as_group().expect("sequence should have group");

            for (tween_index, tween) in seq_group.iter().enumerate() {
                if timeline_group_index == 0 {
                    if seq_group_index == 0 {
                        if tween_index == 0 {
                            let tween = tween.as_clip().expect("group should have clip");
                            if let Tween::Translation(tween) = tween {
                                assert_eq!(tween.x, None); 
                            } else {
                                panic!("not technically an error (groups are unordered) but I'd expect Translation here...");
                            }
                        } else if tween_index == 1 {
                            let tween = tween.as_clip().expect("group should have clip");
                            if let Tween::Translation(tween) = tween {

                                //same time, different bone
                                assert_eq!(tween.x, None); 
                            } else {
                                panic!("not technically an error (groups are unordered) but I'd expect Translation here...");
                            }
                        }
                    } else if seq_group_index == 1 {
                        if tween_index == 0 {
                            let tween = tween.as_clip().expect("group should have clip");
                            if let Tween::Translation(tween) = tween {
                                assert_eq!(tween.x, Some((0.0, 63.0))); 
                            } else {
                                panic!("not technically an error (groups are unordered) but I'd expect Translation here...");
                            }
                        } else if tween_index == 1 {
                            let tween = tween.as_clip().expect("group should have clip");
                            if let Tween::Translation(tween) = tween {

                                //same time, different bone
                                assert_eq!(tween.x, Some((0.0, -80.0))); 
                            } else {
                                panic!("not technically an error (groups are unordered) but I'd expect Translation here...");
                            }
                        }
                    }
                }
            }
        }
    }

    //from here on in it's really just testing for panics
    //printf debugging can help too ;)
    let _active_tweens = timeline.get_active_tweens(5.0 * 1000.0).expect("active tweens");

    //println!("{:#?}", _active_tweens);

    /*
        run some tests on the "hit" animation
    */
    let timeline = animation_to_tween.get("hit").expect("should have run animation");
    assert_eq!(timeline.duration(), 10.0 * 1000.0);
    let groups = timeline.as_group().expect("timeline should have groups");
    assert_eq!(groups.len(), 3); //translation and rotation and color
    for (_timeline_group_index, timeline_group) in groups.iter().enumerate() {
        let sequence = timeline_group.as_sequence().expect("timeline should have sequence");
        assert_eq!(sequence.len(), 1); //all our sequences are 1 item long 

        for (_seq_group_index, seq_group) in sequence.iter().enumerate() {
            let seq_group = seq_group.as_group().expect("sequence should have group");

            for (_tween_index, _tween) in seq_group.iter().enumerate() {
                //too lazy to write tests for this... but it doesn't crash :P
                //println!("\t{:?}", tween);
            }
        }
    }
}