use shipyard::prelude::*;
use shipyard_scenegraph::WorldTransform;
use crate::tick::TickDraw;
use crate::renderer::Renderer;
use crate::camera::Camera;
use crate::components::Renderable;
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