use awsm_web::webgl::Id;

pub struct TextureAtlasFrame {
    pub texture_id: Id,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

pub struct Media {
    pub bg:Bg
}

pub struct Bg {
    pub birds: Vec<TextureAtlasFrame>,
    pub camels: TextureAtlasFrame,
    pub clouds: Vec<TextureAtlasFrame>,
    pub trees: Vec<TextureAtlasFrame>,
    pub pyramid: TextureAtlasFrame
}