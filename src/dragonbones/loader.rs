use wasm_bindgen::prelude::*;
use awsm_web::loaders::fetch;
use awsm_web::webgl::WebGl2Renderer;
use std::collections::HashMap;
use crate::media::loader::media_url;
use crate::textures::{data::Texture, loader::{AtlasStyle, load_texture, get_texture_cell}};
use super::data::*;

pub async fn load(webgl:&mut WebGl2Renderer, base_path:&str) -> Result<DragonBones, JsValue> {
    let (atlas_texture_id, frames, (atlas_width, atlas_height)) = load_texture(webgl, &format!("{}_tex", base_path), Some(AtlasStyle::DragonBones)).await?;

    let skeleton:Skeleton = fetch::json(&media_url(&format!("images/{}_ske.json", base_path))).await?;

    let get_tex_cell = |name:&str| {
        get_texture_cell(name, frames.as_ref().unwrap(), atlas_texture_id, atlas_width, atlas_height)
    };


    let mut textures:HashMap<String, Texture> = HashMap::new();

    frames.as_ref().unwrap().iter().for_each(|frame| {
        let name = &frame.name;
        let texture = get_tex_cell(name);
        textures.insert(name.to_string(), texture);
    });
    
    Ok(DragonBones {
        textures,
        skeleton,
        atlas_width: atlas_width as f64,
        atlas_height: atlas_height as f64
    })
}