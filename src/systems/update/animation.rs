use shipyard::prelude::*;
use wasm_bindgen::prelude::*;
use crate::components::*;
use crate::tick::TickUpdate;

/*
    If there exists an AnimatorEvent::Start for a given entity:
    * Get the AnimationSequences for that entity and event name
    * Add the Animator with that AnimationSequence to the entity
    * Remove the AnimatorEvent from that entity

    AnimatorEvent::Stop is similar, but just removes
*/

#[system(AnimatorEventSys)]
pub fn run(
    tick: Unique<&TickUpdate>, 
    entities_storage: &mut Entities,
    animation_sequences_storage: &mut AnimationSequences,
    mut animator_storage: &mut Animator,
    mut animator_event_storage: &mut AnimatorEvent,
) {
 
    //need to collect the entities for removing the event components
    //If we try to remove it while processing, we'd be
    //either taking multiple mutable references at once, or taking a mutable and immutable ref
    let mut entities_with_events:Vec<EntityId> = Vec::new();

    {
        (&animator_event_storage, &animation_sequences_storage)
            .iter()
            .with_id()
            .for_each(|(entity, (event, sequences_lookup))| {
                entities_with_events.push(entity);
                if let Some((sequence, ending)) = match event {
                    AnimatorEvent::StartByName(name, ending) => {
                        let name:&'static str = name;
                        Some((sequences_lookup.0.get(name).unwrap_throw(), ending))
                    },
                    AnimatorEvent::StartBySequence(sequence, ending) => {
                        Some((sequence, ending))
                    },
                    AnimatorEvent::Stop => {
                        (&mut animator_storage).delete(entity);
                        None
                    },
                } {
                    let animator = Animator::new(sequence.clone(), ending.clone());
                    entities_storage.add_component(&mut animator_storage, animator, entity);
                };
            });
    }

    //delete any events
    for entity in entities_with_events {
        (&mut animator_event_storage).delete(entity);
    }
}

#[system(AnimatorUpdateSys)]
pub fn run(
    tick: Unique<&TickUpdate>, 
    mut animator_storage: &mut Animator,
) {
    (&mut animator_storage)
        .iter()
        .for_each(|animator| {
            animator.playhead += tick.delta;
            if animator.playhead <= animator.sequence.total_duration {
                //TODO:
                //1. get current animation from sequence
                //2. get normalized time within that animation
                //3. get target value(s) with that animation
                //4. alternatively, create specific Modify components, like ModifyTranslation
                //  then those will be applied in those systems, so that we don't need to tie everything up here
            }
        });
}

#[system(AnimatorFinishSys)]
pub fn run(
    tick: Unique<&TickUpdate>, 
    entities_storage: &mut Entities,
    animator_storage: &Animator,
    mut animator_event_storage: &mut AnimatorEvent,
) {
    (&animator_storage)
        .iter()
        .with_id()
        .for_each(|(entity, animator)| {
            if animator.playhead > animator.sequence.total_duration {
                log::info!("{}", animator.playhead);
                let event = match &animator.ending {
                    AnimatorEnding::Loop => {
                        AnimatorEvent::StartBySequence(animator.sequence.clone(), animator.ending.clone())
                    },
                    AnimatorEnding::Remove => {
                        AnimatorEvent::Stop
                    },
                    AnimatorEnding::JumpByName(name, ending) => {
                        AnimatorEvent::StartByName(name.clone(), *ending.clone())
                    },
                    AnimatorEnding::JumpBySequence(sequence, ending) => {
                        AnimatorEvent::StartBySequence(sequence.clone(), *ending.clone())
                    },
                    _ => unimplemented!()
                };
                entities_storage.add_component(&mut animator_event_storage, event, entity);
            }
        });
}