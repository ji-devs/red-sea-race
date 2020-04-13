use derive_deref::{Deref, DerefMut};
use std::collections::HashMap;
use super::timeline::TweenTimeline;

#[derive(Deref, DerefMut, Debug)]
pub struct TweensLookup(pub HashMap<String, TweenTimeline>);