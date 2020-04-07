use serde::{Deserialize};
use std::collections::HashMap;
use crate::textures::data::{Texture, RawFrame};

pub struct DragonBones {
    pub textures:HashMap<String, Texture>,
    pub skeleton:Skeleton
}


#[derive(Deserialize)]
pub struct DragonBonesAtlas {
    #[serde(rename="SubTexture")]
    pub sub_textures: Vec<RawFrame>,
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
    pub animations: Vec<DragonBonesAnimation>,
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
    pub bone: String,
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
    pub transform: Option<BoneTransform>,
    pub pivot: Option<Pivot>
}

#[derive(Deserialize)]
pub struct Pivot {
    pub x: Option<f64>,
    pub y: Option<f64>,
}

#[derive(Deserialize)]
pub struct DragonBonesAnimation {
    pub name: String,
    #[serde(rename="playTimes")]
    pub play_times: Option<usize>,

    pub duration: Option<f64>,

    #[serde(rename="fadeInTime")]
    pub fade_in_time: Option<f64>,

    #[serde(rename="bone")]
    pub bones: Option<Vec<AnimationBone>>,

    #[serde(rename="slot")]
    pub slots: Option<Vec<AnimationSlot>>,
}

#[derive(Deserialize)]
pub struct AnimationBone {
    pub name: String,
    #[serde(rename="translateFrame")]
    pub translation_frames: Option<Vec<AnimationTranslationFrame>>,
    #[serde(rename="rotateFrame")]
    pub rotation_frames: Option<Vec<AnimationRotationFrame>>,
}


#[derive(Deserialize)]
pub struct AnimationTranslationFrame {
    pub duration: Option<f64>,
    #[serde(rename="tweenEasing")]
    pub easing: Option<f64>,
    pub x: Option<f64>,
    pub y: Option<f64>
} 

#[derive(Deserialize)]
pub struct AnimationRotationFrame {
    pub duration: Option<f64>,
    #[serde(rename="tweenEasing")]
    pub easing: Option<f64>,
    #[serde(rename="rotate")]
    pub rotation: Option<f64>,
} 

#[derive(Deserialize)]
pub struct AnimationSlot {
    #[serde(rename="name")]
    pub slot_name: String,
    #[serde(rename="colorFrame")]
    pub color_frames: Option<Vec<AnimationColorFrame>>,
}
#[derive(Deserialize)]
pub struct AnimationColorFrame {
    pub duration: Option<f64>,
    #[serde(rename="tweenEasing")]
    pub easing: Option<f64>,
    pub value: AnimationColorValue,
} 
#[derive(Deserialize)]
pub struct AnimationColorValue {
    #[serde(rename="aM")]
    pub alpha_overlay: Option<f32>,
    #[serde(rename="rM")]
    pub red_overlay: Option<f32>,
    #[serde(rename="gM")]
    pub green_overlay: Option<f32>,
    #[serde(rename="bM")]
    pub blue_overlay: Option<f32>,
    #[serde(rename="aO")]
    pub alpha_offset: Option<f32>,
    #[serde(rename="rO")]
    pub red_offset: Option<f32>,
    #[serde(rename="gO")]
    pub green_offset: Option<f32>,
    #[serde(rename="bO")]
    pub blue_offset: Option<f32>
}