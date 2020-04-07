use shipyard::prelude::*;
use awsm_web::window::get_window_size;
use wasm_bindgen::UnwrapThrowExt;
use shipyard_scenegraph as sg;
use crate::renderer::Renderer;
use crate::camera::Camera;
use crate::media::data::Media;
use crate::tick::{TickBegin, TickUpdate, TickDraw, TickEnd};
use crate::systems;
pub fn init_world(mut renderer:Renderer, media:Media) -> World {
    let world = World::default();

    let (width, height) = get_window_size(&web_sys::window().unwrap_throw()).unwrap_throw();


    world.add_unique(Camera::new(&mut renderer, width, height));
    world.add_unique_non_send_sync(renderer);
    world.add_unique(TickBegin::default());
    world.add_unique(TickUpdate::default());
    world.add_unique(TickDraw::default());
    world.add_unique(TickEnd::default());
    world.add_unique(media);

    
    systems::register_workloads(&world);
    
    sg::init(&world);
    //TODO - tight_pack()

    world
}