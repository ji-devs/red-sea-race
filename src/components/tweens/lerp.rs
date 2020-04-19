use shipyard_scenegraph::{Vec3, Quat};
use nalgebra::UnitQuaternion;
use super::ColorAdjust;

pub trait TweenLerp {
    fn lerp(&self, target: &Self, progress: f64) -> Self;
}

impl TweenLerp for Vec3 {
    fn lerp(&self, target: &Self, progress: f64) -> Self {
        self.lerp(target, progress)
    }
}


impl TweenLerp for Quat {
    fn lerp(&self, target: &Self, progress: f64) -> Self {
        UnitQuaternion::new_unchecked(self.lerp(target, progress))
    }
}


impl TweenLerp for ColorAdjust {
    fn lerp(&self, target: &Self, progress: f64) -> Self {
        Self {
            alpha_overlay: (target.alpha_overlay - self.alpha_overlay) * progress as f32,
            red_overlay: (target.red_overlay - self.red_overlay) * progress as f32,
            green_overlay: (target.green_overlay - self.green_overlay) * progress as f32,
            blue_overlay: (target.blue_overlay - self.blue_overlay) * progress as f32,

            alpha_offset: (target.alpha_offset - self.alpha_offset) * progress as f32,
            red_offset: (target.red_offset - self.red_offset) * progress as f32,
            green_offset: (target.green_offset - self.green_offset) * progress as f32,
            blue_offset: (target.blue_offset - self.blue_offset) * progress as f32,
        }
    }
}