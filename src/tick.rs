use awsm_web::tick::{MainLoop, MainLoopOptions, RafLoop};
use std::rc::Rc;
use shipyard::prelude::*;
use wasm_bindgen::prelude::*;
use crate::systems;

#[derive(Default)]
pub struct TickBegin {
    pub time: f64,
    pub delta: f64
}


#[derive(Default)]
pub struct TickUpdate{
    pub delta: f64
}

#[derive(Default)]
pub struct TickDraw{
    pub interpolation: f64
}

#[derive(Default)]
pub struct TickEnd{
    pub fps: f64,
    pub abort: bool
}

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
            let world = Rc::clone(&world);
            move |time, delta| {
                {
                    let mut tick = world.borrow::<Unique<&mut TickBegin>>();
                    tick.time = time;
                    tick.delta = delta;
                }

                world.run_workload(systems::TICK_BEGIN);
            }
        };

        let update = {
            let world = Rc::clone(&world);
            move |delta| {
                {
                    let mut tick = world.borrow::<Unique<&mut TickUpdate>>();
                    tick.delta = delta;
                }

                world.run_workload(systems::TWEENS);
                world.run_workload(systems::TICK_UPDATE);
            }
        };

        let draw = {
            let world = Rc::clone(&world);
            move |interpolation| {
                {
                    let mut tick = world.borrow::<Unique<&mut TickDraw>>();
                    tick.interpolation = interpolation;
                }

                world.run_workload(systems::TRANSFORMS);
                world.run_workload(systems::TICK_DRAW);
            }
        };

        let end = {
            let world = Rc::clone(&world);
            move |fps, abort| {
                {
                    let mut tick = world.borrow::<Unique<&mut TickEnd>>();
                    tick.fps = fps;
                    tick.abort = abort;
                }

                world.run_workload(systems::TICK_END);
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