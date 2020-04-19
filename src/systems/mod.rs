mod bg;
mod draw;
mod end;
mod motion;
mod remove;
mod tweens;
mod controller;
mod animation;

use shipyard_scenegraph::{self as sg};
use shipyard::prelude::*;

use bg::*;
use controller::*;
use draw::*;
use end::*;
use motion::*;
use remove::*;
use tweens::*;
use animation::*;

pub const TICK_BEGIN: &str = "TICK_BEGIN";
pub const TICK_UPDATE: &str = "TICK_UPDATE";
pub const TICK_DRAW: &str = "TICK_DRAW";
pub const TICK_END: &str = "TICK_END";
pub const TWEENS: &str = "TWEENS";
pub const TRANSFORMS: &str = "TRANSFORMS";

pub fn register_workloads(world: &World) {
    world.add_workload::<ControllerEventSys, _>(TICK_BEGIN);
    world.add_workload::<(TweenEventSys, TweenUpdateSys, TweenFinishSys), _>(TWEENS);
    world.add_workload::<(ControllerUpdateSys, HeroMotionSys, MotionSys, AnimationSys, BgCycleSys, BgSpawnSys, TrashSys), _>(TICK_UPDATE);
    world.add_workload::<(sg::systems::TrsToLocal, sg::systems::LocalToWorld), _>(TRANSFORMS);
    world.add_workload::<TickDrawSys, _>(TICK_DRAW);
    world.add_workload::<(TickEndSys, ControllerClearSys), _>(TICK_END);
}