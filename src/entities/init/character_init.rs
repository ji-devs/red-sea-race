use shipyard::prelude::*;
use crate::media::data::*;
use crate::dragonbones::spawner::spawn;
use crate::components::*;

pub fn init_hero (world:&World) {
    let hero = {
        let media =  world.borrow::<Unique<&Media>>();
        spawn(world, &media.hero)
    };

    world.add_unique(Hero (hero) );

    world.run::<(EntitiesMut, &mut TweenEvent, Unique<&Hero>), _, _>(|(entities, mut animator_events, hero)| {
        entities.add_component(&mut animator_events, TweenEvent::StartByName("run", TweenEnding::Loop), hero.0);
    });
}