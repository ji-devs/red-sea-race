use crate::geometry::{Bounds, BoundsExt};
use awsm_web::webgl::Id;

#[derive(Clone)]
pub struct Texture {
    pub texture_id: Id,
    pub tex_width: usize,
    pub tex_height: usize,
    pub uvs: Uvs,
}

pub static UNIT_UVS: Uvs = [
    0.0, 1.0, // top-left
    0.0, 0.0, //bottom-left
    1.0, 1.0, // top-right
    1.0, 0.0, // bottom-right
];
pub type Uvs = [f32;8];

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