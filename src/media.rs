use awsm_web::webgl::Id;
use serde::{Deserialize};
use crate::geometry::{Bounds, BoundsExt};
use crate::texture::*;

pub struct Media {
    pub bg:Bg
}

pub struct Bg {
    pub layers: Vec<Vec<Texture>>,
    pub birds: Vec<Texture>,
    pub camel: Texture,
    pub clouds: Vec<Texture>,
    pub trees: Vec<Texture>,
    pub pyramid: Texture 
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