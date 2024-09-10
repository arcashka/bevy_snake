use bevy::{
    ecs::system::{lifetimeless::SRes, SystemParamItem},
    log::*,
    pbr::{MeshBindGroups, SetMaterialBindGroup, SetMeshViewBindGroup},
    render::{
        render_phase::{
            PhaseItem, RenderCommand, RenderCommandResult, SetItemPipeline, TrackedRenderPass,
        },
        render_resource::IndexFormat,
    },
};

use super::resources::SnakeMeshInstances;

pub type DrawSnake<M> = (
    SetItemPipeline,
    SetMeshViewBindGroup<0>,
    SetSnakeBindGroup<1>,
    SetMaterialBindGroup<M, 2>,
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
        _: Option<()>,
        snake_instances: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let snake_instances = snake_instances.into_inner();
        let Some(snake) = snake_instances.get(&item.entity()) else {
            error!("snake instance not found");
            return RenderCommandResult::Failure;
        };
        let Some(vertex_buffer) = snake.vertex_buffer.as_ref() else {
            error!("vertex buffer does not exist");
            return RenderCommandResult::Failure;
        };
        let Some(index_buffer) = snake.index_buffer.as_ref() else {
            error!("index buffer does not exist");
            return RenderCommandResult::Failure;
        };
        let Some(indirect_buffer) = snake.indirect_buffer.as_ref() else {
            error!("indirect buffer does not exist");
            return RenderCommandResult::Failure;
        };
        pass.set_vertex_buffer(0, vertex_buffer.slice(..));
        pass.set_index_buffer(index_buffer.slice(..), 0, IndexFormat::Uint32);
        pass.draw_indexed_indirect(indirect_buffer, 0);

        RenderCommandResult::Success
    }
}

pub struct SetSnakeBindGroup<const I: usize>;

impl<P: PhaseItem, const I: usize> RenderCommand<P> for SetSnakeBindGroup<I> {
    type Param = SRes<MeshBindGroups>;
    type ViewQuery = ();
    type ItemQuery = ();

    #[inline]
    fn render<'w>(
        _item: &P,
        _view: (),
        _: Option<()>,
        bind_groups: SystemParamItem<'w, '_, Self::Param>,
        pass: &mut TrackedRenderPass<'w>,
    ) -> RenderCommandResult {
        let bind_groups = bind_groups.into_inner();

        let Some(bind_group) =
            bind_groups.model_only.as_ref()
        else {
            error!(
                "The MeshBindGroups resource wasn't set in the render phase. \
                It should be set by the prepare_mesh_bind_group system.\n\
                This is a bevy bug! Please open an issue."
            );
            return RenderCommandResult::Failure;
        };

        let dynamic_offsets: [u32; 3] = Default::default();
        let offset_count = 0;
        pass.set_bind_group(I, bind_group, &dynamic_offsets[0..offset_count]);

        RenderCommandResult::Success
    }
}

// pub struct CalculateIsosurface;
//
// impl<P: PhaseItem> RenderCommand<P> for CalculateIsosurface {
//     type Param = (
//         SRes<SnakeMeshInstances>,
//         SRes<SnakeComputePipeline>,
//         SRes<PipelineCache>,
//     );
//     type ViewQuery = ();
//     type ItemQuery = ();
//     #[inline]
//     fn render<'w>(
//         _item: &P,
//         _view: (),
//         _: Option<()>,
//         (snakes, compute_pipeline, pipeline_cache): SystemParamItem<'w, '_, Self::Param>,
//         pass: &mut TrackedRenderPass<'w>,
//     ) -> RenderCommandResult {
//         let Some(find_vertices_pipeline) = pipeline_cache.get_compute_pipeline(compute_pipeline.find_vertices_pipeline) else {
//             return RenderCommandResult::Success;
//         };
//         let Some(connect_vertices_pipeline) = pipeline_cache.get_compute_pipeline(compute_pipeline.connect_vertices_pipeline) else {
//             return RenderCommandResult::Success;
//         };
//         let Some(prepare_indirect_buffer_pipeline) = pipeline_cache.get_compute_pipeline(compute_pipeline.prepare_indirect_buffer_pipeline) else {
//             return RenderCommandResult::Success;
//         };
//         for (_, snake) in snakes.iter() {
//             let Some(bind_group) = snake.compute_bind_group.as_ref() else {
//                 error!("missing snake compute bind group");
//             return RenderCommandResult::Success;
//             };
//             pass.set_bind_group(0, bind_group, &[]);
//             pass.set_pipeline(find_vertices_pipeline);
//             pass.dispatch_workgroups(4, 4, 4);
//             pass.set_pipeline(connect_vertices_pipeline);
//             pass.dispatch_workgroups(4, 4, 4);
//             pass.set_pipeline(prepare_indirect_buffer_pipeline);
//             pass.dispatch_workgroups(1, 1, 1);
//         }
//         return RenderCommandResult::Success;
//     }
// }
