use shipyard::prelude::*;
use crate::media::data::*;
use crate::dragonbones::entities::create_entity;
use crate::components::*;

pub fn init_hero (world:&World) {
    let hero = {
        let media =  world.borrow::<Unique<&Media>>();
        create_entity(world, &media.hero)
    };

    world.add_unique(Hero (hero) );

    world.run::<(EntitiesMut, &mut AnimatorEvent, Unique<&Hero>), _, _>(|(entities, mut animator_events, hero)| {
        entities.add_component(&mut animator_events, AnimatorEvent::StartByName("run", AnimatorEnding::Loop), hero.0);
    });
}