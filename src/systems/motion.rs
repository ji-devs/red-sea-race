use shipyard::prelude::*;
use shipyard_scenegraph::Translation;
use crate::components::*;
use crate::tick::TickUpdate;

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