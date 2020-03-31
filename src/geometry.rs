use awsm_web::webgl::{
    WebGl2Renderer, 
    Id, 
    BufferData,
    BufferTarget,
    BufferUsage
};
use wasm_bindgen::prelude::*;

#[derive(Debug)]
pub struct Bounds {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64
}

pub trait BoundsExt {
    fn get_bounds(&self) -> Bounds;
}

pub async fn load_geometry (webgl:&mut WebGl2Renderer) -> Result<Id, JsValue> {
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

 static QUAD_GEOM_UNIT: [f32; 8] = [
    0.0, 1.0, // top-left
    0.0, 0.0, //bottom-left
    1.0, 1.0, // top-right
    1.0, 0.0, // bottom-right
];