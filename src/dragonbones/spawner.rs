use shipyard::prelude::*;
use shipyard_scenegraph::*;
use super::data::DragonBones;
use super::bones::{create_bone_entities, create_slot_lookup};
use super::skin::{create_skin_entities};
use super::animation::create_animations;

use nalgebra::{Unit, Vector3, Quaternion, UnitQuaternion};

pub fn spawn(world:&World, dragonbones:&DragonBones, x:f64, y:f64) -> EntityId {
    let armature = &dragonbones.skeleton.armatures[0];

    let root = spawn_child(world, None, Some(Vector3::new(x, y, 0.0)), None, None, None);


    
    let bone_to_entity = create_bone_entities(&world, root, &armature, dragonbones.atlas_height);
    let slot_to_bone = create_slot_lookup(&armature);
    create_skin_entities(world, &bone_to_entity, &dragonbones.textures, armature, dragonbones.atlas_height);
    create_animations(world, root, &armature, &bone_to_entity, &slot_to_bone, dragonbones.atlas_height);

    /*
    {
        let (parent_storage, child_storage, translation_storage, rotation_storage) = world.borrow::<(&Parent, &Child, &Translation, &Rotation)>();
        let storages = (&parent_storage, &child_storage);

        log::info!("{:?}", storages.debug_tree(root, |e| {
            format!("{:?}: Rotation: {:?}",
                e, 
                &(&rotation_storage).get(e).unwrap().0,
                //get_translation(&(&world_storage).get(e).unwrap().0)
            )
        }));
    }
    */
    root
}