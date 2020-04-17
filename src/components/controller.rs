use std::collections::{HashSet, HashMap};
use derive_deref::{Deref, DerefMut};



/*
    The input that happened between ticks
*/

pub struct ControllerEvent {
    pub down:HashSet<ControllerAction>,
    pub up:HashSet<ControllerAction>,
}
impl ControllerEvent {
    pub fn new() -> Self {
        Self {
            down: HashSet::new(),
            up: HashSet::new(),
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct Controller(pub HashMap<ControllerAction, ControllerState>);

impl Controller {
    pub fn new() -> Self {
        Self (HashMap::new())
    }
}

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum ControllerAction {
    Jump,
    Fire,
    Left,
    Right,
    Down
}

#[derive(Debug, PartialEq)]
pub enum ControllerState {
    Activated,
    Held(f64), //the amount of time its been held
    Released
}