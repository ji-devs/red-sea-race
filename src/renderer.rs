use awsm_web::webgl::{
    Id,
    WebGl2Renderer, 
    WebGlContextOptions,
    get_webgl_context_2,
    GlToggle,
    BlendFactor,
    BufferMask,
    BeginMode,
    AttributeOptions, 
    DataType,
    BufferData,
    BufferTarget,
    BufferUsage,
    VertexArray
};
use web_sys::{HtmlCanvasElement};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use shipyard::prelude::*;
use shipyard_scenegraph::*;
use nalgebra::Matrix4;
use crate::media::loader::load_shaders;
use crate::geometry::upload::upload_quad;
use crate::textures::{data::Texture, uvs::UvFlip};
use crate::components::Renderable;
pub struct Renderer {
    pub canvas: HtmlCanvasElement,
    pub webgl: WebGl2Renderer,
    pub simple_program_id: Id,
    pub vao_id: Id,
    pub geom_buffer_id: Id,
    pub tex_buffer_id: Id,
}

impl Renderer {
    pub async fn new() -> Result<Self, JsValue> {
        let canvas:HtmlCanvasElement = web_sys::window()
            .unwrap_throw()
            .document()
            .unwrap_throw()
            .get_element_by_id("canvas")
            .unwrap_throw()
            .dyn_into()
            .unwrap_throw();

         let gl = get_webgl_context_2(&canvas, Some(&WebGlContextOptions {
            alpha: false,
            ..WebGlContextOptions::default()
        })).unwrap_throw();

        let mut webgl= WebGl2Renderer::new(gl).unwrap_throw();

        webgl.gl.clear_color(1.0, 1.0, 1.0, 1.0);

        let (vertex, fragment) = load_shaders().await.expect("could not load shaders");
        
        cfg_if::cfg_if! {
            if #[cfg(feature = "dev")] {
                let simple_program_id = webgl.compile_program(&vertex, &fragment).unwrap();
            } else {
                let simple_program_id = webgl.compile_program(&vertex, &fragment).unwrap_throw();
            }
        }

        let vao_id = webgl.create_vertex_array()?;

        let geom_buffer_id = upload_quad(&mut webgl).await?;

        let tex_buffer_id = webgl.create_buffer()?;

        webgl.assign_vertex_array(
            vao_id,
            None,
            &[
                VertexArray {
                    attribute_name: "a_geom_vertex",
                    buffer_id: geom_buffer_id,
                    opts: AttributeOptions::new(2, DataType::Float),
                },
                VertexArray {
                    attribute_name: "a_tex_vertex",
                    buffer_id: tex_buffer_id,
                    opts: AttributeOptions::new(2, DataType::Float),
                }
            ],
        )?;

        Ok(Self { canvas, webgl, simple_program_id, vao_id, geom_buffer_id, tex_buffer_id})
    }

    pub fn pre_render(&mut self) {
        let webgl = &mut self.webgl;

        webgl.set_depth_mask(true);
        webgl.toggle(GlToggle::Blend, true);
        webgl.set_blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);
        webgl.toggle(GlToggle::DepthTest, true);
        webgl.clear(&[ BufferMask::ColorBufferBit, BufferMask::DepthBufferBit, ]);
    }

    pub fn render<'a> (&mut self, renderables: impl Shiperator<Item = (&'a Renderable, &'a WorldTransform)>, camera_mat:&Matrix4<f32>) {
        self.pre_render();
        
        let webgl = &mut self.webgl;

        webgl.activate_program(self.simple_program_id).unwrap_throw(); 
        webgl.upload_uniform_mat_4("u_camera", &camera_mat.as_slice()).unwrap_throw();

        let mut model_mat:[f32;16] = [0.0;16];
        renderables.for_each(|(renderable, world_transform)| {
            let webgl = &mut self.webgl;
            let Texture {texture_id, uvs, tex_width, tex_height} = renderable.texture;

            //log::info!("{:?}", world_transform);
            
            //quad scaler
            webgl.upload_uniform_fvals_2("u_quad_scaler", (tex_width as f32, tex_height as f32)).unwrap_throw();

            //model matrix
            world_transform.0.write_to_vf32(&mut model_mat);

            let uvs = match renderable.flip {
                true => uvs.flip(),
                false => uvs
            };

            webgl.upload_buffer(
                self.tex_buffer_id,
                BufferData::new(
                    uvs,
                    BufferTarget::ArrayBuffer,
                    BufferUsage::DynamicDraw,
                ),
            ).unwrap_throw();
            
            webgl.upload_uniform_mat_4("u_model", &model_mat).unwrap_throw();
            webgl.activate_texture_for_sampler(texture_id, "u_sampler").unwrap_throw();
            webgl.activate_vertex_array(self.vao_id).unwrap_throw();
            webgl.draw_arrays(BeginMode::TriangleStrip, 0, 4);
        })
    }
    /*
    pub fn render_bg(&mut self, bg:&Bg, camera_mat: &Matrix4<f32>) {
       

        self.update_vao_data(get_uvs(&bg.pyramid, bg.atlas_size)).unwrap_throw();
        //will eventually be reset per sprite
        let texture_id = bg.pyramid.texture_id;

        let webgl = &mut self.webgl;
        let width = bg.pyramid.width / 3;
        let height= bg.pyramid.height / 3;
        let pos_x = 100.0;
        let pos_y = 100.0;
        
        let scaling_mat = Matrix4::new_nonuniform_scaling(&Vector3::new(
                    width as f32,
                    height as f32,
                    0.0f32,
        ));

        webgl.upload_uniform_fvals_2("u_position", (pos_x, pos_y)).unwrap_throw();
        webgl.upload_uniform_mat_4("u_camera", &camera_mat.as_slice()).unwrap_throw();
        webgl.upload_uniform_mat_4("u_size", &scaling_mat.as_slice()).unwrap_throw();
        webgl.activate_texture_for_sampler(texture_id, "u_sampler").unwrap_throw();
        webgl.activate_vertex_array(self.vao_id).unwrap_throw();
        webgl.draw_arrays(BeginMode::TriangleStrip, 0, 4);

    }
    */
}