mod targets;
mod player;
mod timeline;
mod lookup;
mod events;
#[cfg(test)]
mod tests;

pub use targets::*;
pub use player::*;
pub use timeline::*;
pub use lookup::*;
pub use events::*;


#[derive(Debug, Clone)]
pub enum Tween{
    Translation(Vec3Tween),
    Rotation(QuatTween),
    Scale(Vec3Tween),
    ColorAdjust(ColorTween),
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
}

#[derive(Debug, Clone)]
pub struct TweenInfo {
    pub entity: Option<shipyard::prelude::EntityId>,
    pub duration: f64,
    pub easing: Option<f64>,
}