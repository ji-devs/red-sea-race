use shipyard::prelude::*;
use shipyard_scenegraph::Translation;
use std::collections::HashSet;
use nalgebra::Vector3;
use wasm_bindgen::prelude::*;
use crate::tick::{TickBegin, TickUpdate};
use crate::components::*;
use crate::config::*;

#[system(AnimationSys)]
pub fn run(
    _tick: Unique<&TickUpdate>, 
    entities:Entities, 
    controller: Unique<&Controller>, 
    hero:Unique<&Hero>, 
    hero_jump_controller:Unique<&HeroJumpController>, 
    translations:&Translation,
    mut velocities:&mut Velocity,
    mut gravities:&mut Gravity,
    mut tween_events:&mut TweenEvent,
) {
}