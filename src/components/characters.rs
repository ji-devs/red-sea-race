use crate::textures::data::Texture;
use derive_deref::{Deref, DerefMut};
use shipyard::prelude::*;

#[derive(Deref, DerefMut)]
pub struct Hero(pub EntityId);

#[derive(Deref, DerefMut)]
pub struct HeroJumpController(pub EntityId);