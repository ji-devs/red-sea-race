use shipyard::prelude::*;

pub struct AnimationState {
    pub sequences: Vec<AnimationSequence>
}

pub struct AnimationSequence {
    pub animations: Vec<Animation>
}

pub enum Animation {
    Transform(TransformAnimation),
    Color(ColorAnimation),
}

pub struct TransformAnimation {
    pub entity: EntityId
}

pub struct ColorAnimation {
    pub entity: EntityId
}