use derive_deref::{Deref, DerefMut};


#[derive(Deref, DerefMut)]
pub struct Velocity(pub nalgebra::Vector3<f64>);
