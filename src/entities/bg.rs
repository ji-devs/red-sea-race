use shipyard::prelude::*;
use shipyard_scenegraph as sg;
use nalgebra::Vector3;
use wasm_bindgen::UnwrapThrowExt;
use crate::components::*;
use crate::media::*;
use crate::texture::Texture;
use crate::config::BG_LAYER_DEPTH_START;

/*
    The background layers are like this:

    layer1: [plane1][plane2][planeN]
    layer2: [plane1][plane2][planeN]

    each plane has a `left` which is the EntityId of its neighbor on the left
    plane1's left is the last plane in the layer, creating a cycle
*/

pub fn init_bg_layers(world:&World) {
    let n_layers:Vec<usize> = {
        let media =  world.borrow::<Unique<&Media>>();
        media.bg.layers
            .iter()
            .map(|l| l.len())
            .collect()
    };

    for (layer, n_planes) in n_layers.into_iter().enumerate() {
        //each layer gets positioned in FRONT of the previous
        let layer_depth = BG_LAYER_DEPTH_START + (layer as f64);

        //first create all the entities, so that we can jump into any index
        let plane_entities:Vec<EntityId> = (0..n_planes).into_iter().map(|_| 
            //Translation will be set below when we know the proper x offset
            sg::spawn_child(world, None, None, None, None)
        ).collect();

        //then we can set their components
        let (entities, media, mut translations, mut renderables, mut bg_layers, mut velocities) = 
            world.borrow::<(EntitiesMut, Unique<&Media>, &mut sg::Translation, &mut Renderable, &mut BgLayer, &mut Velocity)>();

        let mut layer_width = 0.0;
        for (index, entity) in plane_entities.iter().enumerate() {
            let left = {
                if index == 0 {
                    plane_entities.last()
                } else {
                    plane_entities.get(index-1)
                }
                .unwrap_throw()
            };
            let left = *left;

            let texture = media.bg.layers[layer][index].clone();
            let tex_width = texture.tex_width;
            entities.add_component(
                (&mut renderables, &mut translations, &mut bg_layers, &mut velocities), 
                (
                    Renderable { texture },
                    //each background 
                    sg::Translation(Vector3::new(layer_width, 0.0, layer_depth)),
                    BgLayer {layer, left},
                    Velocity (Vector3::new(-1.0, 0.0, 0.0))
                ),
                *entity
            );


            layer_width += tex_width as f64;
        }
    }
}