use awsm_web::webgl::Id;
use crate::geometry::{Bounds, BoundsExt};

pub struct TextureAtlasFrame {
    pub texture_id: Id,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl BoundsExt for &TextureAtlasFrame {
    fn get_bounds(&self) -> Bounds {
        let TextureAtlasFrame {x, y, width, height, ..} = self;

        Bounds {
            x: *x as f64,
            y: *y as f64,
            width: *width as f64,
            height: *height as f64,
        }
    }
}

pub struct Media {
    pub bg:Bg
}

pub struct Bg {
    pub atlas_size: (usize, usize),
    pub birds: Vec<TextureAtlasFrame>,
    pub camels: TextureAtlasFrame,
    pub clouds: Vec<TextureAtlasFrame>,
    pub trees: Vec<TextureAtlasFrame>,
    pub pyramid: TextureAtlasFrame
}