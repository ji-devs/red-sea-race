use shipyard::prelude::*;
use shipyard_scenegraph as sg;
use crate::components::*;
use crate::media::*;
use crate::texture::Texture;
use nalgebra::Vector3;

pub fn init(world:&World) {

    let entity = sg::spawn_child(
            world, 
            None,
            None,
            None,
            None
    );

    world.run::<(EntitiesMut, Unique<&Media>, &mut Renderable, &mut BgLayer, &mut NonInteractive), _, _>(|storages| {
        let (entities, media, renderables, bg_layers, non_interactives) = storages;
        create_bg_layer(entity, media.bg.layers[0][0].clone(), bg_layers, renderables, non_interactives, entities);
    });

    let entity = sg::spawn_child(
            world, 
            None,
            Some(Vector3::new(100.0, 0.0, 0.0)),
            None,
            Some(Vector3::new(0.3, 0.3, 0.0)),
            //None,
    );
    world.run::<(EntitiesMut, Unique<&Media>, &mut Renderable, &mut NonInteractive), _, _>(|storages| {
        let (entities, media, renderables, non_interactives) = storages;
        create_bg_sprite(entity, media.bg.pyramid.clone(), renderables, non_interactives, entities);
    });

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

pub fn create_bg_layer(entity:EntityId, texture:Texture,mut bg_layers: ViewMut<BgLayer>,  mut renderables:ViewMut<Renderable>, mut non_interactives:ViewMut<NonInteractive>, entities:EntitiesViewMut) {
    entities.add_component(
        (&mut renderables, &mut non_interactives, &mut bg_layers), 
        (
            Renderable { texture },
            NonInteractive{},
            BgLayer {}
        ),
        entity
    );
}