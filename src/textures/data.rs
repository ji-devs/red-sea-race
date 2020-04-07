use serde::{Deserialize};
use awsm_web::webgl::Id;
use rand::prelude::*;
use crate::geometry::{Bounds, BoundsExt};

#[derive(Clone)]
pub struct Texture {
    pub texture_id: Id,
    pub tex_width: usize,
    pub tex_height: usize,
    pub uvs: super::uvs::Uvs,
}


pub trait RandomTexture {
    fn get_random(&self) -> &Texture;
}

impl <T: AsRef<[Texture]>> RandomTexture for T {
    fn get_random(&self) -> &Texture {
        let mut rng = rand::thread_rng();
        let textures = self.as_ref();
        let index = rng.gen_range(0, textures.len());
        &textures[index]
    }
}

#[derive(Deserialize)]
pub struct RawFrame {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
    pub name: String
}

impl BoundsExt for &RawFrame {
    fn get_bounds(&self) -> Bounds {
        let RawFrame {x, y, width, height, ..} = self;

        Bounds {
            x: *x as f64,
            y: *y as f64,
            width: *width as f64,
            height: *height as f64,
        }
    }
}