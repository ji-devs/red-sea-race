use rand::prelude::*;
use shipyard::prelude::*;
use shipyard_scenegraph::*;
use nalgebra::Vector3;
use crate::components::*;
use crate::media::*;
use crate::config::*;
use crate::textures::data::{Texture, RandomTexture};
//Flat background layers 
#[system(BgCycleSys)]
pub fn run(
    mut translations: &mut Translation,
    bg_layers: &BgLayer,
    renderables: &Renderable,
) {
    let off_screen:Vec<EntityId> = (&translations, &bg_layers, &renderables)
        .iter()
        .with_id()
        .filter(|(_, (pos, _, renderable))| {
            let right_bound = pos.x + (renderable.texture.tex_width as f64); 

            //less than 0.0 is probably fine, but let's give it a bit of padding to be safe
            if right_bound < 2.0 { true } else { false }
        })
        .map(|(entity, _)| entity)
        .collect();

    for entity in off_screen {
        let left = bg_layers[entity].left;
        let left_pos_x = translations[left].x;
        let left_width = renderables[left].texture.tex_width as f64;

        translations[entity].x = left_pos_x + left_width;
    }
    
}

/*
    Background sprites
    The basic idea is that we have a few different layers
    And each laywer spawns some sprite with a velocity
    In order to avoid overcrowding there's a configurable threshhold
    Overall the look and feel is done by tweaking the values in config

    Could definitely be improved! e.g. use scale, fog, semi-transparent, etc.
*/
#[system(BgSpawnSys)]
pub fn run(
    mut entities: &mut Entities,
    mut root: Unique<&TransformRoot>, 
    mut parents: &mut Parent,
    mut children : &mut Child,
    mut translations: &mut Translation,
    mut rotations: &mut Rotation,
    mut scales: &mut Scale,
    mut local_transforms: &mut LocalTransform,
    mut world_transforms: &mut WorldTransform,
    mut dirty_transforms: &mut DirtyTransform,
    media: Unique<&Media>, 
    mut velocities: &mut Velocity,
    mut bg_sprites: &mut BgSprite,
    mut sprites: &mut Sprite,
    mut renderables: &mut Renderable,
) {
    let mut rng = rand::thread_rng();

    //the right bound of each sprite layer, to prevent overlap
    let mut right_bounds:[f64;3] = [0.0;3];

    (&translations, &scales, &bg_sprites, &renderables)
        .iter()
        .for_each(|(pos, _scale, bg_sprite, renderable)| {
            let layer = bg_sprite.layer;
            let right_bound = pos.x + (renderable.texture.tex_width as f64); 

            if right_bound > right_bounds[layer] {
                right_bounds[layer] = right_bound;
            }
        });

    //iterate over each right bound that has enough room to spawn a new object
    right_bounds
        .iter()
        .enumerate()
        .filter(|(layer, right_bound)| {
            let threshhold:f64 = *&BG_SPRITE_SPAWN_THRESHHOLD[*layer];
            let right_bound:f64 = **right_bound;

            if right_bound < ((STAGE_WIDTH as f64) - threshhold) {
                true
            } else {
                false
            }
        })
        .for_each(|(layer, _right_bound)| {
            let vel_minmax = BG_SPRITE_SPAWN_VELOCITY_MINMAX[layer];
            let pos_y_minmax = BG_SPRITE_SPAWN_Y_MINMAX[layer];
            let vel_x = rng.gen_range(vel_minmax.0, vel_minmax.1) * -1.0;
            let pos_y = rng.gen_range(pos_y_minmax.0, pos_y_minmax.1);
            //let scale = rng.gen_range(scale_minmax.0, scale_minmax.1);

            let additional_distance = rng.gen_range(1.0, STAGE_WIDTH);
            let pos = Vector3::new(STAGE_WIDTH + additional_distance, pos_y, BG_SPRITE_DEPTH - (layer as f64));

            let mut flip = false;

            let texture:&Texture = match layer {
                0 => {
                    match rng.gen::<bool>() {
                        true => {
                            flip = rng.gen::<bool>();
                            media.bg.trees.get_random()
                        },
                        false => &media.bg.camel
                    }
                },
                1 => {
                    flip = rng.gen::<bool>();
                    &media.bg.pyramid
                },
                2 => {
                    match rng.gen::<bool>() {
                        true => media.bg.birds.get_random(),
                        false => {
                            flip = rng.gen::<bool>();
                            media.bg.clouds.get_random()
                        }
                    }
                },
                _ => unreachable!()
            };

            let mut sg_storages:TransformHierarchyStoragesMut = (&mut entities, &mut root, &mut parents, &mut children, &mut translations, &mut rotations, &mut scales, &mut local_transforms, &mut world_transforms, &mut dirty_transforms);
            let entity = sg_storages.spawn_child(None, Some(pos), None, None);
            entities.add_component(
                (&mut renderables,&mut sprites, &mut bg_sprites, &mut velocities), 
                (
                    Renderable { texture: texture.clone(), flip},
                    Sprite{},
                    BgSprite {layer},
                    Velocity (Vector3::new(vel_x, 0.0, 0.0))
                ),
                entity
            );
        });
}
/*
    pub birds: Vec<Texture>,
    pub camel: Texture,
    pub clouds: Vec<Texture>,
    pub trees: Vec<Texture>,
    pub pyramid: Texture 
    */