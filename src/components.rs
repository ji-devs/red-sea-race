use crate::texture::*;
use derive_deref::{Deref, DerefMut};
use shipyard::prelude::*;

pub struct Renderable {
    pub texture: Texture 
}

pub struct BgLayer {
    pub layer: usize,
    pub left: EntityId
}

pub struct NonInteractive {}

#[derive(Deref, DerefMut)]
pub struct Velocity(pub nalgebra::Vector3<f64>);
