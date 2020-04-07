mod bg_init;
mod character_init;

use shipyard::prelude::*;

pub fn init(world:&World) {
    bg_init::init_bg_layers(world);
    character_init::init_hero(world);

}