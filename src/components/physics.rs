use crate::textures::data::Texture;
use derive_deref::{Deref, DerefMut};
use shipyard::prelude::*;


#[derive(Deref, DerefMut)]
pub struct Velocity(pub nalgebra::Vector3<f64>);
