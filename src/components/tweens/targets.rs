use shipyard_scenegraph::{Vec3, Quat};
use super::TweenInfo;


#[derive(Debug, Clone)]
pub struct Vec3Tween {
    pub info: TweenInfo,
    pub x: Option<(f64, f64)>,
    pub y: Option<(f64, f64)>,
    pub z: Option<(f64, f64)>,
}

#[derive(Debug, Clone)]
pub struct ScalarTween {
    pub info: TweenInfo,
    pub value: Option<(f64, f64)>,
}

#[derive(Debug, Clone)]
pub struct QuatTween {
    pub info: TweenInfo,
    pub x: Option<(f64, f64)>,
    pub y: Option<(f64, f64)>,
    pub z: Option<(f64, f64)>,
    pub w: Option<(f64, f64)>,
}

#[derive(Debug, Clone)]
pub struct ColorTween {
    pub info: TweenInfo,
    pub alpha_overlay: Option<(f32,f32)>,
    pub red_overlay: Option<(f32,f32)>,
    pub green_overlay: Option<(f32,f32)>,
    pub blue_overlay: Option<(f32,f32)>,

    pub alpha_offset: Option<(f32,f32)>,
    pub red_offset: Option<(f32,f32)>,
    pub green_offset: Option<(f32,f32)>,
    pub blue_offset: Option<(f32,f32)>
}