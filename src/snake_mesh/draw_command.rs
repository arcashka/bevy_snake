use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    log::*,
    pbr::{SetMaterialBindGroup, SetMeshBindGroup, SetMeshViewBindGroup},
    render::render_phase::{
        PhaseItem, RenderCommand, RenderCommandResult, SetItemPipeline, TrackedRenderPass,
    },
};

use super::resources::SnakeMeshInstances;

pub type DrawSnake<M> = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetMaterialBindGroup<M, 1>,
    SetMeshBindGroup<2>,
    DrawSnakeMesh,
);

pub struct DrawSnakeMesh;

impl<P: PhaseItem> RenderCommand<P> for DrawSnakeMesh {
    type Param = SRes<SnakeMeshInstances>;
    type ViewQuery = ();
    type ItemQuery = ();

    #[inline]
    fn render<'w>(
        item: &P,
        _: (),
        _: (),
        snake_instances: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let snake_instances = snake_instances.into_inner();
        info!("render called");
        let Some(snake) = snake_instances.get(&item.entity()) else {
            error!("snake instance not found");
            return RenderCommandResult::Failure;
        };
        let Some(snake_buffer) = snake.buffer.as_ref() else {

            error!("snake buffer does not exist");
            return RenderCommandResult::Failure;
        };
        pass.set_vertex_buffer(0, snake_buffer.slice(..));
        pass.draw(0..snake.buffer_length as u32, 0..1);
        RenderCommandResult::Success
    }
}
