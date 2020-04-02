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
pub const TRANSFORMS: &str = "TRANSFORMS";

pub fn register_workloads(world: &World) {
    world.add_workload::<(TickBeginSys), _>(TICK_BEGIN);
    world.add_workload::<(MotionSys, BgCycleSys), _>(TICK_UPDATE);
    world.add_workload::<(sg::systems::TrsToLocal, sg::systems::LocalToWorld), _>(TRANSFORMS);
    world.add_workload::<(TickDrawSys), _>(TICK_DRAW);
    world.add_workload::<(TickEndSys), _>(TICK_END);
}

#[system(TickBeginSys)]
pub fn run(tick: Unique<&TickBegin>) {
}

#[system(MotionSys)]
pub fn run(
    tick: Unique<&TickUpdate>, 
    mut translations: &mut Translation,
    velocities: &mut Velocity,
) {
    let delta = tick.delta;
    (&mut translations, &velocities).iter().for_each(|(pos, vel)| {
       pos.0 += vel.0 * delta;
    });
}

#[system(BgCycleSys)]
pub fn run(
    mut translations: &mut Translation,
    bg_layers: &BgLayer,
    renderables: &Renderable,
) {
    let off_screen:Vec<EntityId> = (&translations, &renderables)
        .iter()
        .with_id()
        .filter(|(_, (pos, renderable))| {
            let right_bound = pos.x + (renderable.texture.tex_width as f64); 

            //less than 0.0 is probably fine, but let's give it a bit of padding to be safe
            if right_bound < 2.0 { true } else { false }
        })
        .map(|(entity, _)| entity)
        .collect();

    for entity in off_screen {
        let left = bg_layers[entity].left;
        let left_pos_x = translations[left].x;
        let left_width = renderables[left].texture.tex_width as f64;

        translations[entity].x = left_pos_x + left_width;
    }
    
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