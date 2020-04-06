use shipyard::prelude::*;
use crate::tick::TickEnd;

#[system(TickEndSys)]
pub fn run(tick: Unique<&TickEnd>) {
}