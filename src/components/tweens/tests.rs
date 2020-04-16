use super::*;
use std::ptr;
use shipyard::prelude::*;
use shipyard_scenegraph::*;

//TODO - tests for percentage from get_active_tweens()

struct Mock {
    pub world: World,
    pub timeline: TweenTimeline,
}
struct MockRefs <'a> {
    pub group_1: &'a Vec<TweenTimeline>,
    pub sequence_1_1:&'a Vec<TweenTimeline>,
    pub group_1_1_1:&'a Vec<TweenTimeline>,
    pub clip_1_1_1_1:&'a Tween,
    pub clip_1_1_1_2:&'a Tween,
    pub group_1_1_2:&'a Vec<TweenTimeline>,
    pub clip_1_1_2_1:&'a Tween,
    pub clip_1_1_2_2:&'a Tween,
    pub sequence_1_2:&'a Vec<TweenTimeline>,
    pub group_1_2_1:&'a Vec<TweenTimeline>,
    pub clip_1_2_1_1:&'a Tween,
    pub clip_1_2_2: &'a Tween
}

impl Mock {
    fn get_refs(&self) -> MockRefs {
        let group_1 = self.timeline.as_group().expect("top level group");
        
        let sequence_1_1 = group_1[0].as_sequence().expect("sequence_1_1");
        let group_1_1_1 = sequence_1_1[0].as_group().expect("group_1_1_1");
        let clip_1_1_1_1 = group_1_1_1[0].as_clip().expect("clip_1_1_1_1");
        let clip_1_1_1_2 = group_1_1_1[1].as_clip().expect("clip_1_1_1_1");
        let group_1_1_2 = sequence_1_1[1].as_group().expect("group_1_1_2");
        let clip_1_1_2_1 = group_1_1_2[0].as_clip().expect("clip_1_1_2_1");
        let clip_1_1_2_2 = group_1_1_2[1].as_clip().expect("clip_1_1_2_1");
        
        let sequence_1_2 = group_1[1].as_sequence().expect("sequence_1_2");
        let group_1_2_1 = sequence_1_2[0].as_group().expect("group_1_2_1");
        let clip_1_2_1_1 = group_1_2_1[0].as_clip().expect("clip_1_2_1_1");
        let clip_1_2_2 = sequence_1_2[1].as_clip().expect("clip_1_2_2");

        MockRefs {
            group_1,
            sequence_1_1,
            group_1_1_1,
            clip_1_1_1_1,
            clip_1_1_1_2,
            group_1_1_2,
            clip_1_1_2_1,
            clip_1_1_2_2,
            sequence_1_2,
            group_1_2_1,
            clip_1_2_1_1,
            clip_1_2_2
        }
    }
}

#[test]
fn tweens_len() {
    let mock = create_mock();

    let MockRefs {
        group_1,
        sequence_1_1,
        group_1_1_1,
        group_1_1_2,
        sequence_1_2,
        group_1_2_1,
        ..
    } = mock.get_refs();

    assert_eq!(group_1.len(), 2);
    assert_eq!(sequence_1_1.len(), 2);
    assert_eq!(group_1_1_1.len(), 2);
    assert_eq!(group_1_1_2.len(), 2);
    assert_eq!(sequence_1_2.len(), 2);
    assert_eq!(group_1_2_1.len(), 1);
}

#[test]
fn tweens_duration() {
    let mock = create_mock();

    let MockRefs {
        group_1,
        sequence_1_1,
        group_1_1_1,
        group_1_1_2,
        sequence_1_2,
        group_1_2_1,
        clip_1_2_2,
        ..
    } = mock.get_refs();

    assert_eq!(mock.timeline.duration(), 9.0);

    for (index, sequence) in group_1.iter().enumerate() {
        if index == 0 {
            assert_eq!(sequence.duration(), 8.0);
        } else {
            assert_eq!(sequence.duration(), 9.0);
        }
    }

    for group in sequence_1_1.iter() {
        assert_eq!(group.duration(), 4.0);
    }

    for (index, clip) in group_1_1_1.iter().enumerate() {
        if index == 0 {
            assert_eq!(clip.duration(), 4.0);
        } else {
            assert_eq!(clip.duration(), 2.0);
        }
    }

    for (index, clip) in group_1_1_2.iter().enumerate() {
        if index == 0 {
            assert_eq!(clip.duration(), 4.0);
        } else {
            assert_eq!(clip.duration(), 3.0);
        }
    }

    for (index, sub_seq) in sequence_1_2.iter().enumerate() {
        if index == 0 {
            assert_eq!(sub_seq.duration(), 2.0);
        } else {
            assert_eq!(sub_seq.duration(), 7.0);
        }
    }

    for clip in group_1_2_1.iter() {
        assert_eq!(clip.duration(), 2.0);
    }

    assert_eq!(clip_1_2_2.duration(), 7.0);
}


#[test]
fn tweens_playhead() {
    let mock = create_mock();

    let MockRefs {
        clip_1_1_1_1,
        clip_1_1_1_2,
        clip_1_1_2_1,
        clip_1_1_2_2,
        clip_1_2_1_1,
        clip_1_2_2,
        ..
    } = mock.get_refs();

    let timeline = &mock.timeline;

    //the assert!(ptr::eq(...)) stuff is necessary: https://users.rust-lang.org/t/custom-assert-eq-or-vec-eq/40844

    let active_tweens = timeline.get_active_tweens(0.5).expect("has active tweens");
    assert_eq!(active_tweens.len(), 3);

    assert!(ptr::eq(active_tweens[0].1, clip_1_1_1_1));
    assert!(ptr::eq(active_tweens[1].1, clip_1_1_1_2));
    assert!(ptr::eq(active_tweens[2].1, clip_1_2_1_1));

    //0.5 is time into each clip
    assert_eq!(0.5 / active_tweens[0].1.info().duration, active_tweens[0].0);
    assert_eq!(0.5 / active_tweens[1].1.info().duration, active_tweens[1].0);
    assert_eq!(0.5 / active_tweens[2].1.info().duration, active_tweens[2].0);

    let active_tweens = timeline.get_active_tweens(5.0).expect("has active tweens");
    assert_eq!(active_tweens.len(), 3);
    assert!(ptr::eq(active_tweens[0].1, clip_1_1_2_1));
    assert!(ptr::eq(active_tweens[1].1, clip_1_1_2_2));
    assert!(ptr::eq(active_tweens[2].1, clip_1_2_2));

    //5.0 is time into whole timeline, 4.0 is time into first part of sequence
    assert_eq!((5.0 - 4.0) / active_tweens[0].1.info().duration, active_tweens[0].0);
    assert_eq!((5.0 - 4.0) / active_tweens[1].1.info().duration, active_tweens[1].0);

    //5.0 is still time into whole timeline, but here 2.0 is time into first part of sequence
    assert_eq!((5.0 - 2.0) / active_tweens[2].1.info().duration, active_tweens[2].0);

    let active_tweens = timeline.get_active_tweens(8.5).expect("has active tweens");
    assert_eq!(active_tweens.len(), 1);
    assert!(ptr::eq(active_tweens[0].1, clip_1_2_2));

    //8.5 is time into whole timeline, 2.0 is time into first part of sequence
    assert_eq!((8.5 - 2.0) / active_tweens[0].1.info().duration, active_tweens[0].0);

    assert!(timeline.get_active_tweens(9.5).is_none());
}
/*
    Create a situation that looks like this:

    Timeline
        Group_1(total duration = 9)
            Sequence1.1(total duration = 8)
                Group1.1.1(total duration = 4)
                    Clip1.1.1.1: Translation(entity = 1, x = 1, duration = 4)
                    Clip1.1.1.2: Rotation(entity = 1, x = 2, duration = 2)
                Group1.1.2(total duration = 4)
                    Clip1.1.2.1: Translation(entity = 2, x = 3, duration = 4)
                    Clip1.1.2.2: Rotation(entity = 2, x = 4, duration = 3)

            Sequence1.2(total duration = 9)
                Group1.2.1(total duration = 2)
                    Clip1.2.1.1: Color(entity = 3, red_overlay = 1.0, duration = 2)
                Clip1.2.2: Color(entity = 3, red_overlay = 2.0, duration = 7)
*/
fn create_mock() -> Mock {
    let world = World::default();

    shipyard_scenegraph::init(&world);

    let mut group_1:Vec<TweenTimeline> = Vec::new();
    let mut seq_1_1:Vec<TweenTimeline> = Vec::new();
    let mut seq_1_2:Vec<TweenTimeline> = Vec::new();
    let mut group_1_1_1:Vec<TweenTimeline> = Vec::new();
    let mut group_1_1_2:Vec<TweenTimeline> = Vec::new();
    let mut group_1_2_1:Vec<TweenTimeline> = Vec::new();

    let entity_1 = spawn_child(&world, None, None, None, None, None);
    group_1_1_1.push(TweenTimeline::Clip(Tween::Translation(Vec3Tween {
        info: TweenInfo {
            entity: Some(entity_1),
            easing: None,
            duration: 4.0,
        },
        x: Some((0.0, 1.0)),
        y: None,
        z: None
    })));

    group_1_1_1.push(TweenTimeline::Clip(Tween::Rotation(ScalarTween {
        info: TweenInfo {
            entity: Some(entity_1),
            easing: None,
            duration: 2.0,
        },
        value: Some((0.0, 2.0)),
    })));

    seq_1_1.push(TweenTimeline::Group(Box::new(group_1_1_1)));

    let entity_2 = spawn_child(&world, None, None, None, None, None);
    group_1_1_2.push(TweenTimeline::Clip(Tween::Translation(Vec3Tween {
        info: TweenInfo {
            entity: Some(entity_2),
            easing: None,
            duration: 4.0,
        },
        x: Some((0.0, 3.0)),
        y: None,
        z: None
    })));

    group_1_1_2.push(TweenTimeline::Clip(Tween::Rotation(ScalarTween {
        info: TweenInfo {
            entity: Some(entity_2),
            easing: None,
            duration: 3.0,
        },
        value: Some((0.0, 4.0)),
    })));

    seq_1_1.push(TweenTimeline::Group(Box::new(group_1_1_2)));

    let entity_3 = spawn_child(&world, None, None, None, None, None);
    group_1_2_1.push(TweenTimeline::Clip(Tween::ColorAdjust(ColorTween {
        info: TweenInfo {
            entity: Some(entity_3),
            easing: None,
            duration: 2.0,
        },
        alpha_overlay: None,
        red_overlay: Some((0.0, 1.0)),
        green_overlay: None,
        blue_overlay: None,
        alpha_offset: None,
        red_offset: None,
        green_offset: None,
        blue_offset: None,
    })));
    seq_1_2.push(TweenTimeline::Group(Box::new(group_1_2_1)));

    seq_1_2.push(TweenTimeline::Clip(Tween::ColorAdjust(ColorTween {
        info: TweenInfo {
            entity: Some(entity_3),
            easing: None,
            duration: 7.0,
        },
        alpha_overlay: None,
        red_overlay: Some((0.0, 2.0)),
        green_overlay: None,
        blue_overlay: None,
        alpha_offset: None,
        red_offset: None,
        green_offset: None,
        blue_offset: None,
    })));

    group_1.push(TweenTimeline::Sequence(Box::new(seq_1_1)));
    group_1.push(TweenTimeline::Sequence(Box::new(seq_1_2)));

    Mock {
        world,
        timeline: TweenTimeline::Group(Box::new(group_1)),
    }
}