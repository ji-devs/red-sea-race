use shipyard::prelude::*;
use crate::tick::TickEnd;

#[system(TickEndSys)]
pub fn run(_tick: Unique<&TickEnd>) {
}