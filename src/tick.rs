use awsm_web::tick::{MainLoop, MainLoopOptions, RafLoop};
use std::rc::Rc;
use shipyard::prelude::*;
use wasm_bindgen::prelude::*;
use crate::renderer::render;

pub fn start(world:Rc<World>) {
    let game_loop = Box::new(GameLoop::new(world).unwrap_throw());
    std::mem::forget(game_loop);
}

struct GameLoop {
    _raf_loop:RafLoop
}

impl GameLoop {
    fn new(world:Rc<World>) -> Result<Self, JsValue> {
        // loop was ported from https://github.com/IceCreamYou/MainLoop.js#usage
        let begin = {
            let _world = Rc::clone(&world);
            move |_time, _delta| {
                //update input
            }
        };

        let update = {
            let _world = Rc::clone(&world);
            move |_delta| {
                //update motion
            }
        };

        let draw = {
            let world = Rc::clone(&world);
            move |_interpolation| {
                render(&world);
                //render
            }
        };

        let end = {
            move |_fps, _abort| {
            }
        };

        let raf_loop = RafLoop::start({
            let mut main_loop = MainLoop::new(MainLoopOptions::default(), begin, update, draw, end);
            move |ts| {
                main_loop.tick(ts);
            }
        })?;


        Ok(Self{
            _raf_loop: raf_loop
        })
    }
}