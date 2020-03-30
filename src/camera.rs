use nalgebra::Matrix4;

pub struct Camera {
    pub proj_mat: Matrix4<f32>,
    pub stage_width: u32,
    pub stage_height: u32,
}

impl Camera {
    pub fn new(stage_width: u32, stage_height: u32) -> Self {
        Self {
            stage_width, 
            stage_height, 
            proj_mat: create_matrix(stage_width as f32, stage_height as f32)
        }
    }

    pub fn resize(&mut self, stage_width: u32, stage_height: u32) {
        if stage_width != self.stage_width || stage_height != self.stage_height {
            self.proj_mat = create_matrix(stage_width as f32, stage_height as f32);
        }

        self.stage_width = stage_width;
        self.stage_height = stage_height;
    }

}

fn create_matrix(stage_width: f32, stage_height: f32) -> Matrix4<f32> {
    Matrix4::new_orthographic(
        0.0,
        stage_width,
        0.0,
        stage_height,
        0.0,
        1.0,
    )
}


    /*
        if self.last_window_width != window_width || self.last_window_height != window_height {
            self.last_window_width = window_width;
            self.last_window_height = window_height;
        }
    }
    */