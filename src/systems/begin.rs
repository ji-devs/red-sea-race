use shipyard::prelude::*;
use crate::tick::TickBegin;

#[system(TickBeginSys)]
pub fn run(tick: Unique<&TickBegin>) {
}