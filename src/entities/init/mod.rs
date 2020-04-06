mod bg_init;
mod character_init;

use shipyard::prelude::*;
use shipyard_scenegraph as sg;
use nalgebra::Vector3;
use wasm_bindgen::UnwrapThrowExt;
use crate::components::*;
use crate::media::*;
use crate::texture::Texture;

pub fn init(world:&World) {
    bg_init::init_bg_layers(world);
    character_init::init_hero(world);

}