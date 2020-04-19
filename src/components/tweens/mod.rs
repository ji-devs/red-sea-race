mod player;
mod timeline;
mod lookup;
mod events;
mod lerp;
#[cfg(test)]
mod tests;

pub use player::*;
pub use timeline::*;
pub use lookup::*;
pub use events::*;
pub use lerp::*;

use shipyard::prelude::*;
use shipyard_scenegraph::{Vec3, Quat};

#[derive(Debug, Clone)]
pub enum Tween{
    Translation(TweenData<Vec3>),
    Rotation(TweenData<Quat>),
    Scale(TweenData<Vec3>),
    ColorAdjust(TweenData<ColorAdjust>),
}

pub type Easing = f64;

#[derive(Debug, Clone)]
pub struct TweenData<T: Clone + TweenLerp> {
    pub info: TweenInfo,
    pub from: T,
    pub to: T,
}
#[derive(Debug, Clone)]
pub struct TweenInfo {
    pub entity: Option<shipyard::prelude::EntityId>,
    pub duration: f64,
    pub easing: Option<Easing>,
}


impl Tween {
    pub fn duration(&self) -> f64 {
        self.info().duration
    }


    pub fn info(&self) -> &TweenInfo {
        match self {
            Self::Translation(tween) => &tween.info,
            Self::Scale(tween) => &tween.info,
            Self::Rotation(tween) => &tween.info,
            Self::ColorAdjust(tween) => &tween.info,
        }
    }

    //could easily add these with macro... pattern is new_foo() becomes Self::Foo()
    pub fn new_translation(from:Vec3, to:Vec3, duration: f64, entity: Option<EntityId>, easing: Option<Easing>) -> Self {
        Self::Translation(TweenData {
            info: TweenInfo {
                entity,
                duration,
                easing
            },
            from,
            to
        })
    }

    pub fn get_translation_data(&self) -> Option<&TweenData<Vec3>> {
        if let Self::Translation(data) = self {
            Some(data)
        } else {
            None
        }
    }

    pub fn new_rotation(from:Quat, to:Quat, duration: f64, entity: Option<EntityId>, easing: Option<Easing>) -> Self {
        Self::Rotation(TweenData {
            info: TweenInfo {
                entity,
                duration,
                easing
            },
            from,
            to
        })
    }
    pub fn get_rotation_data(&self) -> Option<&TweenData<Quat>> {
        if let Self::Rotation(data) = self {
            Some(data)
        } else {
            None
        }
    }

    pub fn new_scale(from:Vec3, to:Vec3, duration: f64, entity: Option<EntityId>, easing: Option<Easing>) -> Self {
        Self::Scale(TweenData {
            info: TweenInfo {
                entity,
                duration,
                easing
            },
            from,
            to
        })
    }
    pub fn get_scale_data(&self) -> Option<&TweenData<Vec3>> {
        if let Self::Scale(data) = self {
            Some(data)
        } else {
            None
        }
    }
    pub fn new_color_adjust(from:ColorAdjust, to:ColorAdjust, duration: f64, entity: Option<EntityId>, easing: Option<Easing>) -> Self {
        Self::ColorAdjust(TweenData {
            info: TweenInfo {
                entity,
                duration,
                easing
            },
            from,
            to
        })
    }
    pub fn get_color_adjust_data(&self) -> Option<&TweenData<ColorAdjust>> {
        if let Self::ColorAdjust(data) = self {
            Some(data)
        } else {
            None
        }
    }
}

#[derive(Debug, Clone)]
pub struct ColorAdjust {
    pub alpha_overlay: f32,
    pub red_overlay: f32,
    pub green_overlay: f32,
    pub blue_overlay: f32,

    pub alpha_offset: f32,
    pub red_offset: f32,
    pub green_offset: f32,
    pub blue_offset: f32,
}