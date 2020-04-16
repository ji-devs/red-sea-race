use shipyard::prelude::*;
use shipyard_scenegraph::{Translation, Rotation, Scale};
use wasm_bindgen::prelude::*;
use nalgebra::{Unit, Quaternion, UnitQuaternion, Vector3};
use crate::components::*;
use crate::tick::TickUpdate;

/*
    If there exists an TweenEvent::Start for a given entity:
    * Get the TweenSequences for that entity and event name
    * Add the Tween with that TweenSequence to the entity
    * Remove the TweenEvent from that entity

    TweenEvent::Stop is similar, but just removes
*/

#[system(TweenEventSys)]
pub fn run(
    tick: Unique<&TickUpdate>, 
    entities_storage: &mut Entities,
    tweens_lookup_storage: &mut TweensLookup,
    temp: Unique<&HeroJumpController>,
    mut tween_player_storage: &mut TweenPlayer,
    mut tween_event_storage: &mut TweenEvent,
) {

    //need to collect the entities for removing the event components
    //If we try to remove it while processing, we'd be
    //either taking multiple mutable references at once, or taking a mutable and immutable ref
    let mut entities_with_events:Vec<EntityId> = Vec::new();

    {
        (&tween_event_storage)
            .iter()
            .with_id()
            .for_each(|(entity, (event))| {
                entities_with_events.push(entity);
                if let Some((timeline, ending)) = match event {
                    TweenEvent::StartByName(name, ending) => {
                        let name:&'static str = name;
                        let timeline = {
                            (&tweens_lookup_storage)
                                .get(entity)
                                .ok()
                                .and_then(|lookup| {
                                    lookup.0.get(name)
                                })
                                .unwrap_throw()
                        };

                        Some((timeline, ending))
                    },
                    TweenEvent::Start(timeline, ending) => {
                        Some((timeline, ending))
                    },
                    TweenEvent::Stop => {
                        (&mut tween_player_storage).delete(entity);
                        None
                    },
                } {
                    let player = TweenPlayer::new(timeline.clone(), ending.clone());
                    entities_storage.add_component(&mut tween_player_storage, player, entity);
                };
            });
    }

    //delete any events
    for entity in entities_with_events {
        (&mut tween_event_storage).delete(entity);
    }
}

#[system(TweenUpdateSys)]
pub fn run(
    tick: Unique<&TickUpdate>, 
    entities_storage: &mut Entities,
    mut tween_player_storage: &mut TweenPlayer,
    mut translations: &mut Translation,
    mut rotations: &mut Rotation,
    mut scales: &mut Scale,
) {
    
    (&mut tween_player_storage)
        .iter()
        .with_id()
        .for_each(|(player_entity, player)| {
            player.playhead += tick.delta;
            let playhead = {
                if player.playhead > player.duration {
                    player.duration
                } else {
                    player.playhead
                }
            };
            if let Some(active_tweens) = player.timeline.get_active_tweens(playhead) {
                for (progress, tween) in active_tweens {
                    let entity = tween.info().entity.unwrap_or(player_entity);
                    
                    match tween {
                        Tween::Translation(value) => {
                            if let Ok(translation) = (&mut translations).get(entity) {
                                if let Some((start_x, end_x)) = value.x {
                                    translation.x = start_x + ((end_x - start_x) * progress);
                                }
                                if let Some((start_y, end_y)) = value.y {
                                    translation.y = start_y + ((end_y - start_y) * progress);
                                }
                                if let Some((start_z, end_z)) = value.z {
                                    translation.z = start_z + ((end_z - start_z) * progress);
                                }
                            }
                        },
                        Tween::Scale(value) => {
                            if let Ok(scale) = (&mut scales).get(entity) {
                                if let Some((start_x, end_x)) = value.x {
                                    scale.x = (end_x - start_x) * progress;
                                }
                                if let Some((start_y, end_y)) = value.y {
                                    scale.y = (end_y - start_y) * progress;
                                }
                                if let Some((start_z, end_z)) = value.z {
                                    scale.z = (end_z - start_z) * progress;
                                }
                            }
                        },
                        Tween::Rotation(value) => {
                            if let Ok(rotation) = (&mut rotations).get(entity) {
                                if let Some((start_rotation, end_rotation)) = value.value {
                                    let quat = &mut rotation.as_mut_unchecked();
                                    let axis = Unit::new_normalize(Vector3::new(0.0, 0.0, -1.0));
                                    let rotation = start_rotation + ((end_rotation - start_rotation) * progress);
                                    quat.coords = UnitQuaternion::from_axis_angle(&axis, rotation.to_radians()).coords;
                                }
                            }
                        },
                        _ => {}
                    }

                }
            }
        });
}

#[system(TweenFinishSys)]
pub fn run(
    tick: Unique<&TickUpdate>, 
    entities_storage: &mut Entities,
    tween_player_storage: &TweenPlayer,
    mut tween_event_storage: &mut TweenEvent,
) {
    (&tween_player_storage)
        .iter()
        .with_id()
        .for_each(|(entity, player)| {
            if player.playhead > player.duration {
                let event = match &player.ending {
                    TweenEnding::Loop => {
                        //log::info!("looping! {} {}", player.playhead, player.duration);
                        TweenEvent::Start(player.timeline.clone(), player.ending.clone())
                    },
                    TweenEnding::Remove => {
                        TweenEvent::Stop
                    },
                    TweenEnding::SwitchByName(name, ending) => {
                        TweenEvent::StartByName(name, *ending.clone())
                    },
                    TweenEnding::Switch(track, ending) => {
                        TweenEvent::Start(track.clone(), *ending.clone())
                    },
                };
                entities_storage.add_component(&mut tween_event_storage, event, entity);
            }
        });
}