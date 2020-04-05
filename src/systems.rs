use crate::components::*;
use rand::prelude::*;
use shipyard::prelude::*;
use shipyard_scenegraph::{self as sg, *};
use nalgebra::Vector3;
use crate::renderer::Renderer;
use crate::config::*;
use crate::geometry::*;
use crate::camera::Camera;
use crate::media::Media;
use crate::tick::{TickBegin, TickUpdate, TickDraw, TickEnd};

pub const TICK_BEGIN: &str = "TICK_BEGIN";
pub const TICK_UPDATE: &str = "TICK_UPDATE";
pub const TICK_DRAW: &str = "TICK_DRAW";
pub const TICK_END: &str = "TICK_END";
pub const TRANSFORMS: &str = "TRANSFORMS";

pub fn register_workloads(world: &World) {
    world.add_workload::<(TickBeginSys), _>(TICK_BEGIN);
    world.add_workload::<(MotionSys, BgCycleSys, SpawnSys, TrashSys), _>(TICK_UPDATE);
    world.add_workload::<(sg::systems::TrsToLocal, sg::systems::LocalToWorld), _>(TRANSFORMS);
    world.add_workload::<(TickDrawSys), _>(TICK_DRAW);
    world.add_workload::<(TickEndSys), _>(TICK_END);
}

#[system(TickBeginSys)]
pub fn run(tick: Unique<&TickBegin>) {
}

#[system(MotionSys)]
pub fn run(
    tick: Unique<&TickUpdate>, 
    mut translations: &mut Translation,
    velocities: &mut Velocity,
) {
    let delta = tick.delta;
    (&mut translations, &velocities).iter().for_each(|(pos, vel)| {
       pos.0 += vel.0 * delta;
    });
}

#[system(BgCycleSys)]
pub fn run(
    mut translations: &mut Translation,
    bg_layers: &BgLayer,
    renderables: &Renderable,
) {
    let off_screen:Vec<EntityId> = (&translations, &bg_layers, &renderables)
        .iter()
        .with_id()
        .filter(|(_, (pos, layer, renderable))| {
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

#[system(SpawnSys)]
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
   let n_sprites = *&bg_sprites.iter().count();

   let should_spawn = {
       n_sprites == 0
   };

   if should_spawn {
       log::info!("spawning bg sprite!");
        let mut sg_storages:sg::TransformHierarchyStoragesMut = (&mut entities, &mut root, &mut parents, &mut children, &mut translations, &mut rotations, &mut scales, &mut local_transforms, &mut world_transforms, &mut dirty_transforms);

        let entity = sg_storages.spawn_child(None, Some(Vector3::new(STAGE_WIDTH + 1.0, 0.0, BG_SPRITE_DEPTH)) , None, None);
        entities.add_component(
            (&mut renderables,&mut sprites, &mut bg_sprites, &mut velocities), 
            (
                Renderable { texture: media.bg.pyramid.clone() },
                Sprite{},
                BgSprite {},
                Velocity (Vector3::new(-1.0, 0.0, 0.0))
            ),
            entity
        );
        //entities.add_component(&mut bg_sprites, BgSprite{}, entity);
   }
}

//TODO - delete!
#[system(TrashSys)]
pub fn run(
    mut all_storages: &mut AllStorages,
    translations: &Translation,
    sprites: &Sprite,
    renderables: &Renderable,
) {
    let off_screen:Vec<EntityId> = (&translations, &sprites, &renderables)
        .iter()
        .with_id()
        .filter(|(_, (pos, layer, renderable))| {
            let right_bound = pos.x + (renderable.texture.tex_width as f64); 

            //less than 0.0 is probably fine, but let's give it a bit of padding to be safe
            if right_bound < 2.0 { true } else { false }
        })
        .map(|(entity, _)| entity)
        .collect();

    for entity in off_screen {
        log::info!("Delete {:?}", entity);
    }
    
}

#[system(TickDrawSys)]
pub fn run(
    tick: Unique<&TickDraw>, 
    mut renderer: Unique<NonSendSync<&mut Renderer>>, 
    world_transforms: &WorldTransform, 
    camera:Unique<&Camera>, 
    renderables: &Renderable, 
) {
    renderer.render((&renderables, &world_transforms).iter(), &camera.proj_mat);

    //type TransformHierarchyStoragesMut<'a, 'b> = (&'b mut EntitiesViewMut<'a>, &'b UniqueView<'a, TransformRoot>, &'b mut ViewMut<'a, Parent>, &'b mut ViewMut<'a, Child>, &'b mut ViewMut<'a, Translation>, &'b mut ViewMut<'a, Rotation>, &'b mut ViewMut<'a, Scale>, &'b mut ViewMut<'a, LocalTransform>, &'b mut ViewMut<'a, WorldTransform>, &'b mut ViewMut<'a, DirtyTransform>);

}

#[system(TickEndSys)]
pub fn run(tick: Unique<&TickEnd>) {
}