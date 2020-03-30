use shipyard::prelude::*;
use awsm_web::window::get_window_size;
use crate::renderer::Renderer;
use crate::camera::Camera;
use crate::media::Media;
use wasm_bindgen::UnwrapThrowExt;

pub fn init_world(renderer:Renderer, media:Media) -> World {
    let world = World::default();

    let (width, height) = get_window_size(&web_sys::window().unwrap_throw()).unwrap_throw();

    world.add_unique(Camera::new(width, height));
    world.add_unique_non_send_sync(renderer);
    world.add_unique(media);

    //TODO - tight_pack()

    world
}