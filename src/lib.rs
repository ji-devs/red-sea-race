//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

#![feature(drain_filter)]

mod loader;
mod config;
mod path;
mod renderer;
mod dom_events;
mod components;
mod world;
mod media;
mod entities;
mod tick;
mod camera;
mod geometry;
mod textures;
mod systems;
mod dragonbones;

use wasm_bindgen::prelude::*;
use std::rc::Rc;
use renderer::Renderer;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// enable logging and panic hook only during debug builds
cfg_if::cfg_if! {
    if #[cfg(all(feature = "wasm-logger", feature = "console_error_panic_hook", debug_assertions))] {
        fn setup_logger() {
            wasm_logger::init(wasm_logger::Config::default());
            std::panic::set_hook(Box::new(console_error_panic_hook::hook));
            log::info!("rust logging enabled!!!");
        }
    } else {
        fn setup_logger() {
            log::info!("rust logging disabled!"); //<-- won't be seen in release build
        }
    }
}

#[wasm_bindgen(start)]
pub async fn main_js() -> Result<(), JsValue> {

    setup_logger();

    let info_element = get_info_element();

    info_element.set_inner_html("loading shaders...");
    let mut renderer = Renderer::new().await?;
  
    let canvas_element = renderer.canvas.clone();

    info_element.set_inner_html("loading graphics...");
    let media = loader::load_media(&mut renderer.webgl).await?;
    
    info_element.set_inner_html("starting world...");
    let world = Rc::new(world::init_world(renderer, media));

    entities::init::init(&world);
    
    dom_events::start_dom_handlers(Rc::clone(&world));
    

    info_element.remove();
    canvas_element.style().set_property("display", "inline-block").unwrap_throw();

    tick::start(Rc::clone(&world));

    Ok(())

}

fn get_info_element() -> web_sys::Element {
    let document = web_sys::window().unwrap_throw().document().unwrap_throw();
    document.get_element_by_id("info").unwrap_throw()
}