use wasm_bindgen::prelude::*;
use awsm_web::loaders::fetch;
use awsm_web::webgl::WebGl2Renderer;
use crate::config::MEDIA_URL;
use crate::dragonbones;
use crate::textures::loader::{load_texture, load_full_textures, get_texture_cell, AtlasStyle};
use super::data::*;

pub fn media_url(path:&str) -> String {
    format!("{}/{}", MEDIA_URL, path)
}

pub async fn load_shaders() -> Result<(String, String), JsValue> {
    let vertex = fetch::text(&media_url(&"shaders/vertex.glsl")).await?;
    let fragment = fetch::text(&media_url(&"shaders/fragment.glsl")).await?;

    Ok((vertex, fragment))
}


pub async fn load_media(webgl:&mut WebGl2Renderer) -> Result<Media, JsValue> {

    let bg = _load_bg(webgl).await?;

    let hero = dragonbones::loader::load(webgl, "characters/israelite").await?;
    let enemy = dragonbones::loader::load(webgl, "characters/egyptian").await?;

    Ok(Media {
        bg,
        hero,
        enemy
    })

}


async fn _load_bg(webgl:&mut WebGl2Renderer) -> Result<Bg, JsValue> {
    //Load BG Media
    let (atlas_texture_id, frames, atlas_size) = load_texture(webgl, "bg/bg_items", Some(AtlasStyle::Plain)).await?;
    let layers = vec![
        load_full_textures(webgl, vec!["bg/layer_1/bg_1", "bg/layer_1/bg_2", "bg/layer_1/bg_3", "bg/layer_1/bg_4"]).await?,
        load_full_textures(webgl, vec!["bg/layer_2/bg_1", "bg/layer_2/bg_2", "bg/layer_2/bg_3", "bg/layer_2/bg_4"]).await?,
        load_full_textures(webgl, vec!["bg/layer_3/bg_1", "bg/layer_3/bg_2", "bg/layer_3/bg_3", "bg/layer_3/bg_4"]).await?,
    ];

    let get_tex_cell = |name:&str| {
        get_texture_cell(name, frames.as_ref().unwrap(), atlas_texture_id, atlas_size.0, atlas_size.1)
    };


    Ok(Bg {
        layers,

        birds: vec![
            get_tex_cell("bird_1"),
            get_tex_cell("bird_2")
        ],

        camel: get_tex_cell("camels"),
        
        clouds: vec![
            get_tex_cell("cloud_1"),
            get_tex_cell("cloud_2"),
            get_tex_cell("cloud_3"),
        ],

        trees: vec![
            get_tex_cell("palm_tree"),
            get_tex_cell("palm_tree_2"),
        ],

        pyramid: get_tex_cell("pyramid")
    })
}

async fn _load_audio() -> Result<(), JsValue> {
    let err:Result<(), JsValue> = Err(JsValue::from_str("TODO - load audio!"));
    
    err.unwrap_throw();

    Ok(())
}
