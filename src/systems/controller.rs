use shipyard::prelude::*;
use shipyard_scenegraph::Translation;
use std::collections::HashSet;
use nalgebra::Vector3;
use wasm_bindgen::prelude::*;
use crate::tick::{TickBegin, TickUpdate};
use crate::components::*;
use crate::config::*;

#[system(ControllerEventSys)]
pub fn run(tick: Unique<&TickBegin>, mut controller_events:Unique<&mut ControllerEvent>, mut controller: Unique<&mut Controller>) {
    for action in controller_events.up.iter() {
        controller.insert(*action, ControllerState::Released);
    }

    for state in controller.values_mut() {
        if *state == ControllerState::Activated {
            *state = ControllerState::Held(0.0);
        } else if let ControllerState::Held(value) = *state {
            *state = ControllerState::Held(value + tick.delta);
        }
    }

    for action in controller_events.down.iter() {
        //only insert if it's not already in there
        controller
            .entry(*action)
            .or_insert(ControllerState::Activated);
    }

    controller_events.down.clear();
    controller_events.up.clear();
}

#[system(ControllerUpdateSys)]
pub fn run(
    _tick: Unique<&TickUpdate>, 
    entities:Entities, 
    controller: Unique<&Controller>, 
    hero:Unique<&Hero>, 
    hero_jump_controller:Unique<&HeroJumpController>, 
    translations:&Translation,
    mut velocities:&mut Velocity,
    mut tween_events:&mut TweenEvent,
) {

    let entity = hero.0;
    let jump_entity = hero_jump_controller.0;

    let translation = (&translations).get(entity).unwrap_throw();

    let jump_state = controller.get(&ControllerAction::Jump);
    let right_state = controller.get(&ControllerAction::Right);
    let left_state = controller.get(&ControllerAction::Left);
    let down_state = controller.get(&ControllerAction::Down);


    //TODO - maybe the tween should really be on Velocity rather than translation
    if Some(&ControllerState::Activated) == jump_state && translation.y < JUMP_THRESHHOLD {
        entities.add_component(
            &mut tween_events, 
            TweenEvent::Start(
                TweenTimeline::Clip(Tween::Translation(Vec3Tween {
                    info: TweenInfo {
                        entity: Some(entity),
                        easing: None,
                        duration: 400.0
                    },
                    x: None,
                    y: Some((translation.y, translation.y + 500.0)),
                    z: None,
                })),
                TweenEnding::Switch(
                    TweenTimeline::Clip(Tween::Translation(Vec3Tween {
                        info: TweenInfo {
                            entity: Some(entity),
                            easing: None,
                            duration: 400.0
                        },
                        x: None,
                        y: Some((translation.y + 500.0, 0.0)),
                        z: None,
                    })),
                    Box::new(TweenEnding::Remove)
                )
            ), 
            jump_entity
        );
        entities.add_component(&mut tween_events, TweenEvent::StartByName("jump", TweenEnding::SwitchByName( "run", Box::new(TweenEnding::Loop))), entity);
    } else if Some(&ControllerState::Released) == jump_state {
        //TODO - cut jump short?
    }

    //TODO - FIX!!!
    let vel_x = {
        let activated_x = {
            if left_state == Some(&ControllerState::Activated) {
                Some(-1.0)
            } else if right_state == Some(&ControllerState::Activated) {
                Some(1.0)
            } else {
                None
            }
        };
        let released_x = {
            if left_state == Some(&ControllerState::Released) || right_state == Some(&ControllerState::Released) {
                Some(0.0)
            } else {
                None
            }
        };

        let held_x = {
            let left = match left_state {
                Some(state) => {
                    if let ControllerState::Held(time) = state {
                        Some(time)
                    } else {
                        None
                    }
                },
                None => {
                    None
                }
            };
            let right = match right_state {
                Some(state) => {
                    if let ControllerState::Held(time) = state {
                        Some(time)
                    } else {
                        None
                    }
                },
                None => {
                    None
                }
            };

            let values = (left, right);

            match (left, right) {
                //newest wins!
                (Some(left), Some(right)) => {
                    //log::info!("left: {} right: {}", left, right);
                    if left < right {
                        Some(-1.0)
                    } else if right < left {
                        Some(1.0)
                    } else {
                        //what to do when they started being held at same time? eh... don't move anywhere
                        //None would be slightly different - would depend on Release state
                        Some(0.0)
                    }
                },
                (Some(left), None) => {
                    //log::info!("left: {}", left);
                    Some(-1.0)
                },
                (None, Some(right)) => {
                    //log::info!("right: {}", right);
                    Some(1.0)
                },
                (None, None) => None
            }
        };

      
        
        activated_x.or(held_x).or(released_x)

    };

    if let Ok(vel) = (&mut velocities).get(entity) {
        if let Some(x) = vel_x {
            vel.x = x;
        }
    } else {
        let vel = Vector3::new(vel_x.unwrap_or(0.0), 0.0, 0.0);
        entities.add_component(&mut velocities, Velocity(vel), entity);
    }

}

#[system(ControllerClearSys)]
pub fn run(_tick: Unique<&TickBegin>, mut controller:Unique<&mut Controller>) {
    controller.retain(|_, state| {
        *state != ControllerState::Released
    });
}