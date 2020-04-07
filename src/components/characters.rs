use crate::textures::data::Texture;
use derive_deref::{Deref, DerefMut};
use shipyard::prelude::*;

#[derive(Deref)]
pub struct Hero(pub EntityId);