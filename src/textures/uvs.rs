use crate::geometry::bounds::Bounds;

pub static UNIT_UVS: Uvs = [
    0.0, 1.0, // top-left
    0.0, 0.0, //bottom-left
    1.0, 1.0, // top-right
    1.0, 0.0, // bottom-right
];

pub trait UvFlip {
    fn flip(&self) -> Self;
}

pub type Uvs = [f32;8];

impl UvFlip for Uvs {
    fn flip(&self) -> Self {

        let tl_x = self[0];
        let tl_y = self[1];
        let bl_x = self[2];
        let bl_y = self[3];
        let tr_x = self[4];
        let tr_y = self[5];
        let br_x = self[6];
        let br_y = self[7];

        [tr_x, tr_y, br_x, br_y, tl_x, tl_y, bl_x, bl_y]
    }
}
pub fn get_uvs(atlas_width: usize, atlas_height: usize, bounds: &Bounds) -> Uvs {

    let atlas_width = atlas_width as f64;
    let atlas_height = atlas_height as f64;

    let Bounds {x, y, width, height} = *bounds; 

    //Bounds are assuming 0,0 is bottom-left
    //Texture atlas assumes 0,0 is top-right
    //So we need to invert the y axis
    let mut x1 = x;
    let mut y1 = atlas_height - (y + height); 
    let mut x2 = x + width; 
    let mut y2 = atlas_height - y; 
 
    //Normalize relative to full image width/height
    x1 /= atlas_width;
    y1 /= atlas_height;
    x2 /= atlas_width;
    y2 /= atlas_width;

    //Get the corners, just for the sake of clarity
    //Might as well do the casting here too
    let tl = (x1 as f32, y2 as f32);
    let bl = (x1 as f32, y1 as f32);
    let tr = (x2 as f32, y2 as f32);
    let br = (x2 as f32, y1 as f32);

    //return it as a straight array
    [tl.0, tl.1, bl.0, bl.1, tr.0, tr.1, br.0, br.1]
}