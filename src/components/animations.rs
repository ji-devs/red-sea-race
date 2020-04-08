/*
    The animation methodology is pretty much just to support our dragonbones needs
    though it could definitely be expanded!

    Overall the concept is that animation data is lightweight and easily cloned
    It gets instantiated per-instance and per-animation

    Would be nice to instead make it reference-based, but that's tricky
    Since each animation targets a specific entity (bone, texture, etc.)
*/

use shipyard::prelude::*;
use derive_deref::{Deref, DerefMut};
use std::collections::HashMap;


//Make it easier to manage animators in systems
//This event is always exactly just one at a time (most recent wins)
#[derive(Debug)]
pub enum AnimatorEvent {
    //The name of the animation sequence, and ending style
    StartByName(&'static str, AnimatorEnding),
    StartBySequence(AnimationSequence, AnimatorEnding),
    Stop
}

//To play a given animation sequence
#[derive(Debug)]
pub struct Animator {
    pub sequence: AnimationSequence,
    pub index: usize,
    pub playhead: f64,
    pub state: AnimatorState,
    pub ending: AnimatorEnding
}

impl Animator {
    pub fn new(sequence: AnimationSequence, ending: AnimatorEnding) -> Self {
        Self {
            sequence,
            index: 0,
            playhead: 0.0,
            state: AnimatorState::Playing,
            ending
        }
    }
}

#[derive(Debug)]
pub enum AnimatorState {
    Playing,
    Paused,
    Finished
}


#[derive(Debug, Clone)]
pub enum AnimatorEnding {
    Loop,
    Remove,
    JumpByName(&'static str, Box<AnimatorEnding>), //the name of the next sequence
    JumpBySequence(AnimationSequence, Box<AnimatorEnding>) //the name of the next sequence
}

//Just a lookup to get animation sequence by name
#[derive(Deref, DerefMut, Debug)]
pub struct AnimationSequences(pub HashMap<String, AnimationSequence>);

#[derive(Debug, Clone)]
pub struct AnimationSequence {
    pub animations: Vec<Animation>,
    pub total_duration: f64
}

#[derive(Debug, Clone)]
pub struct Animation {
    pub entity: EntityId,
    pub target: AnimationTarget,
    pub duration: f64,
    pub easing: Option<f64>
}

#[derive(Debug, Clone)]
pub enum AnimationTarget {
    Translation(TranslationAnimationTarget),
    Rotation(RotationAnimationTarget),
    Color(ColorAnimationTarget),
}

#[derive(Debug, Clone)]
pub struct TranslationAnimationTarget {
    pub x: Option<f64>,
    pub y: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct RotationAnimationTarget {
    pub rotation: Option<f64>,
}

#[derive(Debug, Clone)]
pub struct ColorAnimationTarget {
    pub alpha_overlay: Option<f32>,
    pub red_overlay: Option<f32>,
    pub green_overlay: Option<f32>,
    pub blue_overlay: Option<f32>,

    pub alpha_offset: Option<f32>,
    pub red_offset: Option<f32>,
    pub green_offset: Option<f32>,
    pub blue_offset: Option<f32>
}