pub mod loader;
pub mod uvs;
use awsm_web::webgl::Id;
use rand::prelude::*;

#[derive(Clone)]
pub struct Texture {
    pub texture_id: Id,
    pub tex_width: usize,
    pub tex_height: usize,
    pub uvs: uvs::Uvs,
}


pub trait RandomTexture {
    fn get_random(&self) -> &Texture;
}

impl <T: AsRef<[Texture]>> RandomTexture for T {
    fn get_random(&self) -> &Texture {
        let mut rng = rand::thread_rng();
        let textures = self.as_ref();
        let index = rng.gen_range(0, textures.len());
        &textures[index]
    }
}