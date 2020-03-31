use crate::components::*;
use rand::prelude::*;
use shipyard::prelude::*;
use shipyard_scenegraph::{self as sg, *};
use crate::renderer::Renderer;
use crate::config::*;
use crate::geometry::*;
use crate::camera::Camera;
use crate::media::Media;
use crate::tick::{TickBegin, TickUpdate, TickDraw, TickEnd};

pub const TICK_BEGIN: &str = "TICK_BEGIN";
pub const TICK_UPDATE: &str = "TICK_UPDATE";
pub const TICK_DRAW: &str = "TICK_DRAW";
pub const TICK_END: &str = "TICK_END";
pub const TRANSFORMS: &str = "TICK_END";

pub fn register_workloads(world: &World) {
    world.add_workload::<(TickBeginSys), _>(TICK_BEGIN);
    world.add_workload::<(TickUpdateSys), _>(TICK_UPDATE);
    world.add_workload::<(sg::systems::TrsToLocal, sg::systems::LocalToWorld), _>(TRANSFORMS);
    world.add_workload::<(TickDrawSys), _>(TICK_DRAW);
    world.add_workload::<(TickEndSys), _>(TICK_END);
}

#[system(TickBeginSys)]
pub fn run(tick: Unique<&TickBegin>) {
}

#[system(TickUpdateSys)]
pub fn run(
    tick: Unique<&TickUpdate>, 
    mut entities:EntitiesMut,
    mut translations: &mut Translation,
) {
    let delta = tick.delta;
    (&mut translations).iter().for_each(|t| {
        (*t).0.x -= delta * 1.0;
    });
}

#[system(TickDrawSys)]
pub fn run(
    tick: Unique<&TickDraw>, 
    mut renderer: Unique<NonSendSync<&mut Renderer>>, 
    world_transforms: &WorldTransform, 
    camera:Unique<&Camera>, 
    renderables: &Renderable, 
) {
    renderer.render((&renderables, &world_transforms).iter(), &camera.proj_mat);
}

#[system(TickEndSys)]
pub fn run(tick: Unique<&TickEnd>) {
}