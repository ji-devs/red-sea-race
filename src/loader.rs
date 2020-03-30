use wasm_bindgen::prelude::*;
use web_sys::HtmlImageElement;
use awsm_web::loaders::fetch;
use awsm_web::webgl::{WebGl2Renderer, Id, TextureTarget, SimpleTextureOptions, PixelFormat, WebGlTextureSource};
use serde::Deserialize;
use std::rc::Rc;
use shipyard::prelude::*;
use crate::path;
use crate::media::*;
use crate::renderer::Renderer;

pub async fn load_shaders() -> Result<(String, String), JsValue> {
    let vertex = fetch::text(&path::media_url(&"shaders/vertex.glsl")).await?;
    let fragment = fetch::text(&path::media_url(&"shaders/fragment.glsl")).await?;

    Ok((vertex, fragment))
}

#[derive(Deserialize)]
struct RawFrame {
    x: u32,
    y: u32,
    width: u32,
    height: u32,
    name: String
}

pub async fn load_media(webgl:&mut WebGl2Renderer) -> Result<Media, JsValue> {
    //Load BG Media
    let (texture_id, frames) = load_texture_atlas(webgl, "bg/bg_items").await?;
    let get_frame = |name:&str| get_tex_frame(name, texture_id, &frames);

    let bg = Bg {
        birds: vec![
        get_frame("bird_1"),
        get_frame("bird_2")
        ],

        camels: get_frame("camels"),
        
        clouds: vec![
            get_frame("cloud_1"),
            get_frame("cloud_2"),
            get_frame("cloud_3"),
        ],

        trees: vec![
            get_frame("palm_tree"),
            get_frame("palm_tree_2"),
        ],

        pyramid: get_frame("pyramid")
    };

    Ok(Media {bg})

}

async fn load_audio() -> Result<(), JsValue> {
    let err:Result<(), JsValue> = Err(JsValue::from_str("TODO - load audio!"));
    
    err.unwrap_throw();

    Ok(())
}

async fn load_texture_atlas(webgl:&mut WebGl2Renderer, src:&str) -> Result<(Id, Vec<RawFrame>), JsValue> {
    let img = fetch::image(&path::media_url(&format!("images/{}.png", src))).await?;
    let frames:Vec<RawFrame> = fetch::json(&path::media_url(&format!("images/{}.json", src))).await?;

    let texture_id = webgl.create_texture()?;
    webgl.assign_simple_texture(
        texture_id,
        TextureTarget::Texture2d,
        &SimpleTextureOptions {
            pixel_format: PixelFormat::Rgba,
            ..SimpleTextureOptions::default()
        },
        &WebGlTextureSource::ImageElement(&img),
    )?;

    Ok((texture_id, frames))
    
}

fn get_tex_frame<'a> (name:&str, texture_id: Id, frames:&'a Vec<RawFrame>) -> TextureAtlasFrame {
    let RawFrame {width, height, x, y, ..} = *frames.iter().find(|frame| frame.name == name).expect(&format!("{} doesn't exist!", name));

    TextureAtlasFrame {
        texture_id,
        width,
        height,
        x,
        y 
    }
}