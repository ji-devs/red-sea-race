use wasm_bindgen::prelude::*;
use awsm_web::loaders::fetch;
use awsm_web::webgl::{WebGl2Renderer, Id, TextureTarget, SimpleTextureOptions, PixelFormat, WebGlTextureSource};
use crate::path;
use crate::media::*;
use crate::geometry::BoundsExt;
use super::{Texture, uvs::*};

pub enum AtlasStyle {
    Plain,
    DragonBones
}

pub async fn load_texture(webgl:&mut WebGl2Renderer, src:&str, atlas_style:Option<AtlasStyle>) -> Result<(Id, Option<Vec<RawFrame>>, (usize, usize)), JsValue> {
    let img = fetch::image(&path::media_url(&format!("images/{}.png", src))).await?;
    let tex_size = (img.width() as usize, img.height() as usize);
    let frames:Option<Vec<RawFrame>> =  match atlas_style {
        Some(AtlasStyle::Plain) => {
            let frames:Vec<RawFrame> = fetch::json(&path::media_url(&format!("images/{}.json", src))).await?;
            Some(frames)
        },
        Some(AtlasStyle::DragonBones) => {
            log::info!("{}", src);
            let atlas:DragonBonesAtlas = fetch::json(&path::media_url(&format!("images/{}.json", src))).await?;
            Some(atlas.sub_textures)
        },
        None => None
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
pub async fn load_full_textures(webgl:&mut WebGl2Renderer, srcs:Vec<&'static str>) -> Result<Vec<Texture>, JsValue> {
    let mut textures:Vec<Texture> = Vec::new();

    for src in srcs {
        let (texture_id, _, tex_size) = load_texture(webgl, src, None).await?;
        let uvs = UNIT_UVS;
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

pub fn get_texture_cell(name:&str, frames:&Vec<RawFrame>, texture_id: Id, atlas_width: usize, atlas_height: usize) -> Texture {
    let raw_frame = frames.iter().find(|frame| frame.name == name).expect(&format!("{} doesn't exist!", name));

    let uvs = get_uvs(atlas_width, atlas_height, &raw_frame.get_bounds());

    Texture {
        texture_id,
        uvs,
        tex_width: raw_frame.width,
        tex_height: raw_frame.height,
    }
}