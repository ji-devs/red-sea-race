mod begin;
mod update;
mod draw;
mod end;

use shipyard_scenegraph::{self as sg};
use shipyard::prelude::*;
use begin::*;
use update::*;
use draw::*;
use end::*;

pub const TICK_BEGIN: &str = "TICK_BEGIN";
pub const TICK_UPDATE: &str = "TICK_UPDATE";
pub const TICK_DRAW: &str = "TICK_DRAW";
pub const TICK_END: &str = "TICK_END";
pub const TRANSFORMS: &str = "TRANSFORMS";

pub fn register_workloads(world: &World) {
    world.add_workload::<TickBeginSys, _>(TICK_BEGIN);
    world.add_workload::<(MotionSys, BgCycleSys, BgSpawnSys, TrashSys), _>(TICK_UPDATE);
    //world.add_workload::<(TrashSys), _>(TICK_UPDATE);
    world.add_workload::<(sg::systems::TrsToLocal, sg::systems::LocalToWorld), _>(TRANSFORMS);
    world.add_workload::<TickDrawSys, _>(TICK_DRAW);
    world.add_workload::<TickEndSys, _>(TICK_END);
}