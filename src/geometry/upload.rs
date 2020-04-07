use awsm_web::webgl::{WebGl2Renderer, BufferData, BufferTarget, BufferUsage, Id};
use wasm_bindgen::prelude::*;
use super::data::QUAD_GEOM_UNIT;

pub async fn upload_quad (webgl:&mut WebGl2Renderer) -> Result<Id, JsValue> {
    let buffer_id = webgl.create_buffer()?;

    webgl.upload_buffer(
        buffer_id,
        BufferData::new(
            &QUAD_GEOM_UNIT,
            BufferTarget::ArrayBuffer,
            BufferUsage::StaticDraw,
        ),
    )?;

    Ok(buffer_id)
 }