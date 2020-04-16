use shipyard::prelude::*;
use std::collections::HashSet;
use crate::tick::TickBegin;
use crate::components::*;

#[system(ControllerStartSys)]
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

#[system(ControllerEndSys)]
pub fn run(_tick: Unique<&TickBegin>, mut controller:Unique<&mut Controller>) {
    for (action, state) in controller.iter() {
        log::info!("{:?} {:?}", action, state);
    }
    controller.retain(|action, state| {
        *state != ControllerState::Released
    });
}
/*
#[system(KeyboardMapperSys)]
pub fn run(_tick: Unique<&TickBegin>, keyboard:Unique<&Keyboard>, mut controller:Unique<&mut Controller>) {
    let keystate
    if Some(KeyboardState::Active) == keyboard.state.get("ArrowUp").cloned() || Some(KeyboardState::Active) == keyboard.state.get("W").cloned() {
        log::info!("key up pressed...");
    }
}

#[system(ControllerSys)]
pub fn run(_tick: Unique<&TickBegin>, controller:Unique<&Controller>) {
    match *controller {
        Controller::Fire => {
            log::info!("fire!");
        },
        Controller::Jump => {
            log::info!("jump!");
        },
        _ => {},
    }
}
*/