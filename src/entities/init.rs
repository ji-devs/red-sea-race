use shipyard::prelude::*;
use shipyard_scenegraph as sg;
use nalgebra::Vector3;
use wasm_bindgen::UnwrapThrowExt;
use crate::components::*;
use crate::media::*;
use crate::texture::Texture;
use super::bg::init_bg_layers;

pub fn init(world:&World) {
    init_bg_layers(world);

}