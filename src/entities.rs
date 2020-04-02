use shipyard::prelude::*;
use shipyard_scenegraph as sg;
use nalgebra::Vector3;
use wasm_bindgen::UnwrapThrowExt;
use crate::components::*;
use crate::media::*;
use crate::texture::Texture;

pub fn init(world:&World) {
    init_bg_layers(world);

}

/*
    The background layers are like this:

    layer1: [plane1][plane2][planeN]
    layer2: [plane1][plane2][planeN]

    each plane has a `left` which is the EntityId of its neighbor on the left
    plane1's left is the last plane in the layer, creating a cycle
*/

fn init_bg_layers(world:&World) {
    let n_layers:Vec<usize> = {
        let media =  world.borrow::<Unique<&Media>>();
        media.bg.layers
            .iter()
            .map(|l| l.len())
            .collect()
    };

    for (layer, n_planes) in n_layers.into_iter().enumerate() {
        //first create all the entities, so that we can jump into any index
        let plane_entities:Vec<EntityId> = (0..n_planes).into_iter().map(|_| 
            sg::spawn_child(world, None, Some(Vector3::new(0.0, 0.0, 0.0 - layer as f64)), None, None)
        ).collect();

        //then we can set their components
        let (entities, media, mut translations, mut renderables, mut bg_layers, mut non_interactives, mut velocities) = 
            world.borrow::<(EntitiesMut, Unique<&Media>, &mut sg::Translation, &mut Renderable, &mut BgLayer, &mut NonInteractive, &mut Velocity)>();

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
            //TODO OH NO- THIS PANICS!!!!!!!!
            /*
            entities.add_component(
                (&mut renderables, &mut translations, &mut non_interactives, &mut bg_layers, &mut velocities), 
                (
                    Renderable { texture },
                    sg::Translation(Vector3::new(0.0, 0.0, 0.0)),
                    NonInteractive{},
                    BgLayer {layer, left},
                    Velocity (Vector3::new(-1.0, 0.0, 0.0))
                ),
                *entity
            );
            */

            //so for now add the new components only
            entities.add_component(
                (&mut renderables, &mut non_interactives, &mut bg_layers, &mut velocities), 
                (
                    Renderable { texture },
                    NonInteractive{},
                    BgLayer {layer, left},
                    Velocity (Vector3::new(-5.0, 0.0, 0.0))
                ),
                *entity
            );

            //another example of how it panics!
            //entities.add_component(&mut translations, sg::Translation(Vector3::new(layer_width, 0.0, 0.0)), *entity);
            
            //so for now update the component instead
            *(&mut translations).get(*entity).unwrap() = sg::Translation(Vector3::new(layer_width, 0.0, 0.0));

            layer_width += tex_width as f64;
        }
    }
}
pub fn create_bg_sprite(entity:EntityId, texture:Texture, mut renderables:ViewMut<Renderable>, mut non_interactives:ViewMut<NonInteractive>, entities:EntitiesViewMut) {
    entities.add_component(
        (&mut renderables, &mut non_interactives), 
        (
            Renderable { texture },
            NonInteractive{}
        ),
        entity
    );
}