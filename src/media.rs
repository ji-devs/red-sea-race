use awsm_web::webgl::Id;
use serde::{Deserialize};
use std::collections::HashMap;
use crate::geometry::{Bounds, BoundsExt};
use crate::texture::*;

pub struct Media {
    pub bg:Bg,
    pub hero: Character,
    pub enemy: Character,
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


pub struct Character {
    pub textures:HashMap<String, Texture>,
    pub skeleton:Skeleton
}

//https://docs.egret.com/dragonbones/docs/dbLibs/5foramt

#[derive(Deserialize)]
pub struct Skeleton {
    #[serde(rename="frameRate")]
    pub frame_rate: f64,

    #[serde(rename="armature")]
    pub armatures: Vec<Armature>,
}

#[derive(Deserialize)]
pub struct Armature {
    #[serde(rename="frameRate")]
    pub frame_rate: f64,
    pub aabb:Aabb,
    #[serde(rename="bone")]
    pub bones: Vec<Bone>,
    #[serde(rename="slot")]
    pub slots: Vec<Slot>,
    #[serde(rename="skin")]
    pub skins: Vec<Skin>,
    #[serde(rename="animation")]
    pub animations: Vec<Animation>,
}

#[derive(Deserialize)]
pub struct Aabb {
    pub width: f64,
    pub height: f64
}

#[derive(Deserialize, Debug)]
pub struct Bone {
    pub name: String,
    #[serde(rename="parent")]
    pub parent_name: Option<String>,
    pub transform: Option<BoneTransform>
}

#[derive(Deserialize, Debug)]
pub struct BoneTransform {
    pub x: Option<f64>,
    pub y: Option<f64>,
    #[serde(rename="skX")]
    pub skew_x: Option<f64>,
    #[serde(rename="skY")]
    pub skew_y: Option<f64>,
    #[serde(rename="scX")]
    pub scale_x: Option<f64>,
    #[serde(rename="scY")]
    pub scale_y: Option<f64>,
}

#[derive(Deserialize)]
pub struct Slot {
    pub name: String,
    #[serde(rename="parent")]
    pub parent_name: Option<String>,
}

#[derive(Deserialize)]
pub struct Skin {
    #[serde(rename="slot")]
    pub slots: Vec<SkinSlot>,
}

#[derive(Deserialize)]
pub struct SkinSlot {
    #[serde(rename="name")]
    pub bone_name: String,
    pub display: Vec<SkinSlotDisplay>
}
#[derive(Deserialize)]
pub struct SkinSlotDisplay {
    #[serde(rename="name")]
    pub texture_name: String,
    pub transform: Option<BoneTransform>

}

#[derive(Deserialize)]
pub struct Animation {
}