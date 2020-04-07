use crate::textures::data::Texture;
use crate::dragonbones::data::DragonBones;

pub struct Media {
    pub bg:Bg,
    pub hero: DragonBones,
    pub enemy: DragonBones,
}

pub struct Bg {
    pub layers: Vec<Vec<Texture>>,
    pub birds: Vec<Texture>,
    pub camel: Texture,
    pub clouds: Vec<Texture>,
    pub trees: Vec<Texture>,
    pub pyramid: Texture 
}

