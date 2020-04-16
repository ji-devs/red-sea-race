use shipyard::prelude::*;
use shipyard_scenegraph::Translation;
use crate::components::*;
use crate::tick::TickUpdate;

#[system(MotionSys)]
pub fn run(
    tick: Unique<&TickUpdate>, 
    velocities: &mut Velocity,
    mut translations: &mut Translation,
) {
    let delta = tick.delta;
    (&mut translations, &velocities).iter().for_each(|(pos, vel)| {
       pos.0 += vel.0 * delta;
    });
}

#[system(HeroMotionSys)]
pub fn run(
    mut translations: &mut Translation,
    mut velocities: &mut Velocity,
    hero: Unique<&Hero>,
) {
    if let Ok((translation, velocity)) = (&mut translations, &mut velocities).get(hero.0) {
        if (translation.y + velocity.y) < 0.0 {
            translation.y = 0.0;
            velocity.y = 0.0;
        }
    }
}
