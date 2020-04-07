#[derive(Debug)]
pub struct Bounds {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64
}

pub trait BoundsExt {
    fn get_bounds(&self) -> Bounds;
}