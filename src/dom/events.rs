use web_sys::{window, Event, MouseEvent};
use awsm_web::window::get_window_size;
use std::rc::Rc;
use shipyard::prelude::*;
use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use wasm_bindgen::UnwrapThrowExt;
use crate::renderer::Renderer;
use crate::camera::Camera;

pub fn start_dom_handlers(world:Rc<World>) {
    let window = window().unwrap_throw();
    
    let canvas = world.borrow::<Unique<NonSendSync<&mut Renderer>>>().canvas.clone();

    let on_resize = {
        let window = window.clone();
        let world = Rc::clone(&world);
        move |_: &web_sys::Event| {
            let (width, height) = get_window_size(&window).unwrap_throw();
            
            let mut renderer = world.borrow::<Unique<NonSendSync<&mut Renderer>>>();
            
            world.borrow::<Unique<&mut Camera>>()
                .resize(&mut renderer, width, height);
        }
    };

    on_resize(&web_sys::Event::new("").unwrap_throw());

    EventListener::new(&window, "resize", on_resize).forget();


    EventListener::new(&canvas, "pointerdown", {
        let world = Rc::clone(&world);
        move |event:&Event| {
            let camera = world.borrow::<Unique<&mut Camera>>();
            let event = event.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
            let (touch_x, touch_y) = get_point(&camera, &event);
            log::info!("pointerdown at {},{}", touch_x, touch_y);
        }
    }).forget();

    EventListener::new(&canvas, "pointerup", {
        let world = Rc::clone(&world);
        move |event:&Event| {
            let camera = world.borrow::<Unique<&mut Camera>>();
            let event = event.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
            let (touch_x, touch_y) = get_point(&camera, &event);
            log::info!("pointerup at {},{}", touch_x, touch_y);
        }
    }).forget();
}

fn get_point(camera:&Camera, event:&MouseEvent) -> (f64, f64) {
 
    let viewport = &camera.viewport;

    let (client_x, client_y) = (event.client_x() as f64, event.client_y() as f64);
   
    let (canvas_x, canvas_y) = (client_x, ((camera.window_height as f64) - client_y));

    let (viewport_x, viewport_y) = (canvas_x - viewport.x, canvas_y - viewport.y);
   
    let (world_x, world_y) = (viewport_x / viewport.scale, viewport_y / viewport.scale);

    (world_x, world_y)
}