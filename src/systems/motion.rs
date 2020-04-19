use shipyard::prelude::*;
use shipyard_scenegraph::Translation;
use crate::components::*;
use crate::tick::TickUpdate;
use crate::config::*;

#[system(MotionSys)]
pub fn run(
    tick: Unique<&TickUpdate>, 
    mut velocities: &mut Velocity,
    mut gravities: &mut Gravity,
    mut translations: &mut Translation,
) {
    let delta = tick.delta;
    (&mut translations, &velocities).iter().for_each(|(pos, vel)| {
       pos.0 += vel.0 * delta;
    });
    
    (&mut velocities, &mut gravities)
        .iter()
        .for_each(|(vel, grav)| {
            vel.y -= grav.0 * delta; 
            grav.0 += (GRAVITY_INCREASE* delta);
        });
}

#[system(HeroMotionSys)]
pub fn run(
    entities:Entities, 
    mut translations: &mut Translation,
    mut velocities: &mut Velocity,
    mut gravities:&mut Gravity,
    mut tween_events:&mut TweenEvent,
    hero: Unique<&Hero>,
) {
    let entity = hero.0;
    if let Ok((translation, velocity)) = (&mut translations, &mut velocities).get(entity) {
        if velocity.x < 0.0 && (translation.x + velocity.x) < 0.0 {
            translation.x = 0.0;
        } else if velocity.x > 0.0 && (translation.x + velocity.x) > STAGE_WIDTH {
            translation.x = STAGE_WIDTH;
        }

        if velocity.y < 0.0 && (translation.y + velocity.y) <= 0.0 {
            translation.y = 0.0;
            velocity.y = 0.0;
            entities.add_component(&mut tween_events, TweenEvent::StartByName("run", TweenEnding::Loop), entity);
            gravities.remove(entity);
        } else if (translation.y == 0.0 && velocity.y > 0.0) {
            entities.add_component(&mut tween_events, TweenEvent::StartByName("jump", TweenEnding::Loop), entity);
        }
    }
}
