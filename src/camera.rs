use nalgebra::Matrix4;
use awsm_web::webgl::{ResizeStrategy, PartialWebGlViewport};
use crate::renderer::Renderer;
use crate::config::{STAGE_WIDTH, STAGE_HEIGHT, STAGE_RATIO, CAMERA_DEPTH};
pub struct Camera {
    pub proj_mat: Matrix4<f32>,
    pub window_width: u32,
    pub window_height: u32,
    pub viewport: Viewport
}

#[derive(Debug)]
pub struct Viewport {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub scale: f64
}

impl Camera {
    pub fn new(renderer: &mut Renderer, window_width: u32, window_height: u32) -> Self {
        let viewport = Self::set_viewport(renderer, window_width, window_height);

        let proj_mat = Matrix4::new_orthographic( 0.0, STAGE_WIDTH as f32, 0.0, STAGE_HEIGHT as f32, 0.01, CAMERA_DEPTH as f32);
        
        Self { window_width, window_height, viewport, proj_mat }
    }

    pub fn resize(&mut self, renderer: &mut Renderer, window_width: u32, window_height: u32) {
        if window_width != self.window_width || window_height != self.window_height {
            let viewport = Self::set_viewport(renderer, window_width, window_height);
            self.viewport = viewport;
            self.window_width = window_width;
            self.window_height = window_height;
        }
    }

    fn set_viewport(renderer: &mut Renderer, window_width: u32, window_height: u32) -> Viewport {
        let viewport = scale_to_fit(window_width as f64, window_height as f64);
        renderer.webgl.resize(ResizeStrategy::Canvas(window_width, window_height));
        renderer.webgl.gl.awsm_viewport(viewport.x as u32, viewport.y as u32, viewport.width as u32, viewport.height as u32);

        viewport
    }
}

fn scale_to_fit(viewport_width: f64, viewport_height: f64) -> Viewport {
    let mut width = viewport_width;
    let mut height = viewport_height;

    let viewport_ratio = viewport_width / viewport_height;
    //compare viewport ratio to art resolution
    if viewport_ratio > STAGE_RATIO {
        width = height * STAGE_RATIO;
    } else {
        height = width / STAGE_RATIO;
    }

    //offset in order to center it in the area
    let x = (viewport_width - width) / 2.0;
    let y = (viewport_height - height) / 2.0;

    //how much it shrank
    let scale = width / STAGE_WIDTH;

    Viewport { x, y, width, height, scale} 
}