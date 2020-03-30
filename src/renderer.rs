use awsm_web::webgl::{
    Id,
    WebGlRenderer, 
    WebGl2Renderer, 
    WebGlCommon, 
    WebGlContextOptions,
    get_webgl_context_2,
    GlToggle,
    BlendFactor,
    BufferMask,
    BeginMode
};
use web_sys::{HtmlCanvasElement};
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use shipyard::prelude::*;
use nalgebra::{Matrix4, Vector3};
use crate::media::*;
use crate::camera::Camera;
use crate::loader::load_shaders;
use crate::geometry::load_geometry;

pub struct Renderer {
    pub canvas: HtmlCanvasElement,
    pub webgl: WebGl2Renderer,
    pub simple_program_id: Id,
    pub quad_vao_id: Id
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
        let simple_program_id = webgl.compile_program(&vertex, &fragment).unwrap_throw();

        let quad_vao_id = load_geometry(&mut webgl).await?;

        Ok(Self { canvas, webgl, simple_program_id, quad_vao_id})
    }

    pub fn pre_render(&mut self) {
        let webgl = &mut self.webgl;
        webgl.set_depth_mask(false);
        webgl.toggle(GlToggle::Blend, true);
        webgl.set_blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);
        webgl.clear(&[ BufferMask::ColorBufferBit, BufferMask::DepthBufferBit, ]);
    }

    //TODO - see note in render() below
    //Also TODO - render just the given frame ;)
    //Also TODO - instancing :P
    pub fn render_bg(&mut self, bg:&Bg, camera_mat: &Matrix4<f32>) {
        let webgl = &mut self.webgl;
       
        webgl.activate_program(self.simple_program_id).unwrap_throw(); 

        //will eventually be reset per sprite
        let texture_id = bg.pyramid.texture_id;
        let width = bg.pyramid.width;
        let height= bg.pyramid.height;
        let pos_x = 0.0;
        let pos_y = 0.0;
        
        let scaling_mat = Matrix4::new_nonuniform_scaling(&Vector3::new(
                    width as f32,
                    height as f32,
                    0.0f32,
        ));

        webgl.upload_uniform_fvals_2("u_position", (pos_x, pos_y)).unwrap_throw();
        webgl.upload_uniform_mat_4("u_camera", &camera_mat.as_slice()).unwrap_throw();
        webgl.upload_uniform_mat_4("u_size", &scaling_mat.as_slice()).unwrap_throw();
        webgl.activate_texture_for_sampler(texture_id, "u_sampler").unwrap_throw();
        webgl.activate_vertex_array(self.quad_vao_id).unwrap_throw();
        webgl.draw_arrays(BeginMode::TriangleStrip, 0, 4);
    }
}

pub fn render(world:&World) {
    let mut renderer = world.borrow::<Unique<NonSendSync<&mut Renderer>>>();
    let media = world.borrow::<Unique<&Media>>();
    let camera= world.borrow::<Unique<&Camera>>();

    renderer.pre_render();

    //TODO - just make Renderables which own their own TextureAtlas frame (or are just regular ids)
    //In other words this should be system/component-driven
    //Though layering matters so systems need to be executed left->right in order of z-depth
    renderer.render_bg(&media.bg, &camera.proj_mat);
}

