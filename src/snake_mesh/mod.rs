mod components;
mod draw_command;
mod gpu_systems;
mod phase_item;
mod pipeline;
mod resources;

use bevy::{
    prelude::*,
    render::{
        render_phase::{sort_phase_system, AddRenderCommand, DrawFunctions},
        render_resource::SpecializedMeshPipelines,
        Render, RenderApp, RenderSet,
    },
};

pub use components::SnakeMesh;

pub struct SnakeMeshPlugin;
impl Plugin for SnakeMeshPlugin {
    fn build(&self, app: &mut App) {
        app.sub_app_mut(RenderApp)
            .init_resource::<DrawFunctions<phase_item::SnakePhaseItem>>()
            .add_systems(
                ExtractSchedule,
                (
                    gpu_systems::extract_snakes,
                    gpu_systems::extract_snake_camera_phases,
                ),
            )
            .add_systems(
                Render,
                (
                    // sort_phase_system::<phase_item::SnakePhaseItem>.in_set(RenderSet::PhaseSort),
                    gpu_systems::queue_snake_meshes.in_set(RenderSet::Queue),
                    // batch_and_prepare_render_phase::<
                    //     phase_item::SnakePhaseItem,
                    //     pipeline::SnakePipeline,
                    // >,
                    gpu_systems::prepare_buffers.in_set(RenderSet::PrepareResources),
                ),
            )
            .add_render_command::<phase_item::SnakePhaseItem, draw_command::DrawSnake>()
            .init_resource::<SpecializedMeshPipelines<pipeline::SnakePipeline>>()
            .init_resource::<resources::SnakeMeshInstances>();
    }

    fn finish(&self, app: &mut App) {
        app.sub_app_mut(RenderApp)
            .init_resource::<pipeline::SnakePipeline>();
    }
}