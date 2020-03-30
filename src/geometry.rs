use awsm_web::webgl::{
    WebGl2Renderer, 
    Id, 
    VertexArray, 
    AttributeOptions, 
    DataType,
    BufferData,
    BufferTarget,
    BufferUsage
};
use wasm_bindgen::prelude::*;

pub async fn load_geometry(webgl:&mut WebGl2Renderer) -> Result<Id, JsValue> {
    let buffer_id = webgl.create_buffer()?;

    webgl.upload_buffer(
        buffer_id,
        BufferData::new(
            &QUAD_GEOM_UNIT,
            BufferTarget::ArrayBuffer,
            BufferUsage::StaticDraw,
        ),
    )?;

    let vao_id = webgl.create_vertex_array()?;

    webgl.assign_vertex_array(
        vao_id,
        None,
        &[VertexArray {
            attribute_name: "a_vertex",
            buffer_id,
            opts: &AttributeOptions::new(2, DataType::Float),
        }],
    )?;

    Ok(vao_id)
 }

 static QUAD_GEOM_UNIT: [f32; 8] = [
    0.0, 1.0, // top-left
    0.0, 0.0, //bottom-left
    1.0, 1.0, // top-right
    1.0, 0.0, // bottom-right
];