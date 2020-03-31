use wasm_bindgen::prelude::*;
use awsm_web::loaders::fetch;
use awsm_web::webgl::{WebGl2Renderer, Id, TextureTarget, SimpleTextureOptions, PixelFormat, WebGlTextureSource};
use futures::{
    future::TryFutureExt,
    try_join,
    future::try_join_all,
    future::join_all,
    FutureExt

};
use serde::Deserialize;
use crate::geometry::BoundsExt;
use crate::path;
use crate::media::*;
use crate::texture::{self, Texture};

pub async fn load_shaders() -> Result<(String, String), JsValue> {
    let vertex = fetch::text(&path::media_url(&"shaders/vertex.glsl")).await?;
    let fragment = fetch::text(&path::media_url(&"shaders/fragment.glsl")).await?;

    Ok((vertex, fragment))
}


pub async fn load_media(webgl:&mut WebGl2Renderer) -> Result<Media, JsValue> {
    //Load BG Media
    let (atlas_texture_id, frames, atlas_size) = load_texture(webgl, "bg/bg_items", true).await?;
    let layers = vec![
        load_full_textures(webgl, vec!["bg/layer_1/bg_1", "bg/layer_1/bg_2", "bg/layer_1/bg_3", "bg/layer_1/bg_4"]).await?,
        load_full_textures(webgl, vec!["bg/layer_2/bg_1", "bg/layer_2/bg_2", "bg/layer_2/bg_3", "bg/layer_2/bg_4"]).await?,
        load_full_textures(webgl, vec!["bg/layer_3/bg_1", "bg/layer_3/bg_2", "bg/layer_3/bg_3", "bg/layer_3/bg_4"]).await?,
    ];

    let get_tex_cell = |name:&str| {
        get_texture_cell(name, frames.as_ref().unwrap(), atlas_texture_id, atlas_size.0, atlas_size.1)
    };


    let bg = Bg {
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
    };

    Ok(Media {bg})

}

async fn _load_audio() -> Result<(), JsValue> {
    let err:Result<(), JsValue> = Err(JsValue::from_str("TODO - load audio!"));
    
    err.unwrap_throw();

    Ok(())
}

async fn load_texture(webgl:&mut WebGl2Renderer, src:&str, as_atlas:bool) -> Result<(Id, Option<Vec<RawFrame>>, (usize, usize)), JsValue> {
    let img = fetch::image(&path::media_url(&format!("images/{}.png", src))).await?;
    let tex_size = (img.width() as usize, img.height() as usize);
    let frames:Option<Vec<RawFrame>> = if as_atlas {
        fetch::json(&path::media_url(&format!("images/{}.json", src))).await?
    } else {
        None
    };

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

    Ok((texture_id, frames, tex_size))
    
}

//TODO - make work concurrently 
async fn load_full_textures(webgl:&mut WebGl2Renderer, srcs:Vec<&'static str>) -> Result<Vec<Texture>, JsValue> {
    let mut textures:Vec<Texture> = Vec::new();

    for src in srcs {
        let (texture_id, _, tex_size) = load_texture(webgl, src, false).await?;
        let uvs = texture::UNIT_UVS;
        textures.push(Texture {
            texture_id,
            uvs,
            tex_width: tex_size.0,
            tex_height: tex_size.1 
        });
    }

    Ok(textures)
    /*
    try_join_all(
        srcs.into_iter().map(move |src| {
            load_texture(webgl, src, false)
            .map(|res| {
                res.map(|(texture_id, _, tex_size)| {
                    let uvs = texture::UNIT_UVS;
                    Texture {
                        texture_id,
                        uvs,
                        tex_width: tex_size.0,
                        tex_height: tex_size.1 
                    }
                })
            })
        })
        
    ).await
    */

}

fn get_texture_cell(name:&str, frames:&Vec<RawFrame>, texture_id: Id, atlas_width: usize, atlas_height: usize) -> Texture {
    let raw_frame = frames.iter().find(|frame| frame.name == name).expect(&format!("{} doesn't exist!", name));

    let uvs = texture::get_uvs(atlas_width, atlas_height, &raw_frame.get_bounds());

    Texture {
        texture_id,
        uvs,
        tex_width: raw_frame.width,
        tex_height: raw_frame.height,
    }
}