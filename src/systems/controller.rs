use shipyard::prelude::*;
use shipyard_scenegraph::Translation;
use std::collections::HashSet;
use nalgebra::Vector3;
use wasm_bindgen::prelude::*;
use crate::tick::{TickBegin, TickUpdate};
use crate::components::*;

#[system(ControllerEventSys)]
pub fn run(_tick: Unique<&TickBegin>, mut controller_events:Unique<&mut ControllerEvent>, mut controller: Unique<&mut Controller>) {
    for action in controller_events.up.iter() {
        controller.insert(*action, ControllerState::Released);
    }

    for state in controller.values_mut() {
        if *state == ControllerState::Activated {
            *state = ControllerState::Held;
        }
    }

    for action in controller_events.down.iter() {
        controller
            .entry(*action)
            .and_modify(|e| *e = ControllerState::Held)
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
    let mut vel_x:Option<f64> = None;
    let mut vel_y:Option<f64> = None;

    let entity = hero.0;
    let jump_entity = hero_jump_controller.0;

    let translation = (&translations).get(entity).unwrap_throw();

    let jump_state = controller.get(&ControllerAction::Jump);
    let right_state = controller.get(&ControllerAction::Right);
    let left_state = controller.get(&ControllerAction::Left);
    let down_state = controller.get(&ControllerAction::Down);

    //TODO - also change animation
    if Some(&ControllerState::Activated) == jump_state && translation.y == 0.0 {
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
       // vel_y = Some(-1.0);
    }

    if Some(&ControllerState::Activated) == right_state { 
        vel_x = Some(1.0);
    } else if Some(&ControllerState::Activated) == left_state { 
        vel_x = Some(-1.0);
    } else if Some(&ControllerState::Released) == right_state || Some(&ControllerState::Released) == left_state {
        vel_x = Some(0.0);
    }


    if let Ok(vel) = (&mut velocities).get(entity) {
        if let Some(y) = vel_y {
            vel.y = y;
        }
        if let Some(x) = vel_x {
            vel.x = x;
        }
    } else {
        let vel = Vector3::new(vel_x.unwrap_or(0.0), vel_y.unwrap_or(0.0), 0.0);
        entities.add_component(&mut velocities, Velocity(vel), entity);
    }

}

#[system(ControllerClearSys)]
pub fn run(_tick: Unique<&TickBegin>, mut controller:Unique<&mut Controller>) {
    controller.retain(|_, state| {
        *state != ControllerState::Released
    });
}