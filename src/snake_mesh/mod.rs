mod components;
mod draw_command;
mod gpu_systems;
mod node;
mod pipelines;
mod resources;

use bevy::{
    core_pipeline::core_3d::{
        graph::{Core3d, Node3d},
        AlphaMask3d, Opaque3d, Transmissive3d, Transparent3d,
    },
    prelude::*,
    render::{
        batching::batch_and_prepare_render_phase, render_graph::RenderGraphApp,
        render_phase::AddRenderCommand, render_resource::SpecializedMeshPipelines, Render,
        RenderApp, RenderSet,
    },
};

pub use components::SnakeMesh;

use node::{SnakeComputeNode, SnakeComputeNodeLabel};

#[derive(Default)]
pub struct SnakeMeshPlugin;

impl Plugin for SnakeMeshPlugin {
    fn build(&self, app: &mut App) {
        app.sub_app_mut(RenderApp)
            .add_systems(ExtractSchedule, (gpu_systems::extract_snakes,))
            .add_systems(
                Render,
                (
                    gpu_systems::queue_material_snakes::<StandardMaterial>.in_set(RenderSet::Queue),
                    gpu_systems::create_snake_buffers.in_set(RenderSet::PrepareResources),
                    (
                        batch_and_prepare_render_phase::<
                            Transmissive3d,
                            pipelines::SnakeMaterialPipeline<StandardMaterial>,
                        >,
                        batch_and_prepare_render_phase::<
                            Transparent3d,
                            pipelines::SnakeMaterialPipeline<StandardMaterial>,
                        >,
                        batch_and_prepare_render_phase::<
                            Opaque3d,
                            pipelines::SnakeMaterialPipeline<StandardMaterial>,
                        >,
                        batch_and_prepare_render_phase::<
                            AlphaMask3d,
                            pipelines::SnakeMaterialPipeline<StandardMaterial>,
                        >,
                    )
                        .in_set(RenderSet::PrepareResources),
                    gpu_systems::prepare_snake_compute_bind_groups
                        .in_set(RenderSet::PrepareBindGroups),
                ),
            )
            .add_render_command::<Transmissive3d, draw_command::DrawSnake<StandardMaterial>>()
            .add_render_command::<Transparent3d, draw_command::DrawSnake<StandardMaterial>>()
            .add_render_command::<Opaque3d, draw_command::DrawSnake<StandardMaterial>>()
            .add_render_command::<AlphaMask3d, draw_command::DrawSnake<StandardMaterial>>()
            .init_resource::<SpecializedMeshPipelines<pipelines::SnakeMaterialPipeline<StandardMaterial>>>()
            .init_resource::<resources::SnakeMeshInstances>()
            .add_render_graph_node::<SnakeComputeNode>(Core3d, SnakeComputeNodeLabel)
            .add_render_graph_edge(Core3d, SnakeComputeNodeLabel, Node3d::StartMainPass);
    }

    fn finish(&self, app: &mut App) {
        app.sub_app_mut(RenderApp)
            .init_resource::<pipelines::SnakeMaterialPipeline<StandardMaterial>>()
            .init_resource::<pipelines::SnakeComputePipeline>();
    }
}
