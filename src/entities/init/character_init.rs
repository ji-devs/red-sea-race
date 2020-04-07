use shipyard::prelude::*;
use crate::media::*;
use crate::dragonbones::*;
use crate::components::*;

pub fn init_hero (world:&World) {
    let hero = {
        let media =  world.borrow::<Unique<&Media>>();
        create_dragonbones_entity(world, &media.hero)
    };

    world.add_unique(Hero (hero) );
}