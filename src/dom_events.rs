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

            world.borrow::<Unique<NonSendSync<&mut Renderer>>>()
                .webgl
                .resize(width, height);

            world.borrow::<Unique<&mut Camera>>()
                .resize(width, height);
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
    (event.client_x() as f64, ((camera.stage_height as i32) - event.client_y()) as f64)
}