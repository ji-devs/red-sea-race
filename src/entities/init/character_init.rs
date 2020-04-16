use shipyard::prelude::*;
use crate::media::data::*;
use crate::dragonbones::spawner::spawn;
use crate::components::*;

pub fn init_hero (world:&World) {
    let hero = {
        let media =  world.borrow::<Unique<&Media>>();
        spawn(world, &media.hero, 100.0, 0.0)
    };

    world.add_unique(Hero (hero) );
    let entity = world.borrow::<EntitiesMut>().add_entity((), ());
    world.add_unique(HeroJumpController (entity) );

    world.run::<(EntitiesMut, &mut TweenEvent, Unique<&Hero>), _, _>(|(entities, mut tween_events, hero)| {
        entities.add_component(&mut tween_events, TweenEvent::StartByName("run", TweenEnding::Loop), hero.0);
    });
}