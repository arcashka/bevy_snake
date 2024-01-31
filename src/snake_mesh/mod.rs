mod components;
mod draw_command;
mod gpu_systems;
mod pipeline;
mod resources;

use std::hash::Hash;
use std::marker::PhantomData;

use bevy::{
    core_pipeline::core_3d::{AlphaMask3d, Opaque3d, Transmissive3d, Transparent3d},
    prelude::*,
    render::{
        render_phase::AddRenderCommand, render_resource::SpecializedMeshPipelines, Render,
        RenderApp, RenderSet,
    },
};

pub use components::SnakeMesh;

pub struct SnakeMeshPlugin<M: Material> {
    pub _marker: PhantomData<M>,
}

impl Default for SnakeMeshPlugin<StandardMaterial> {
    fn default() -> Self {
        Self {
            _marker: PhantomData,
        }
    }
}

impl<M: Material> Plugin for SnakeMeshPlugin<M>
where
    M::Data: PartialEq + Eq + Hash + Clone,
{
    fn build(&self, app: &mut App) {
        app.sub_app_mut(RenderApp)
            .add_systems(ExtractSchedule, (gpu_systems::extract_snakes,))
            .add_systems(
                Render,
                (
                    gpu_systems::queue_material_snakes::<M>.in_set(RenderSet::Queue),
                    gpu_systems::prepare_buffers.in_set(RenderSet::PrepareResources),
                ),
            )
            .add_render_command::<Transmissive3d, draw_command::DrawSnake<M>>()
            .add_render_command::<Transparent3d, draw_command::DrawSnake<M>>()
            .add_render_command::<Opaque3d, draw_command::DrawSnake<M>>()
            .add_render_command::<AlphaMask3d, draw_command::DrawSnake<M>>()
            .init_resource::<SpecializedMeshPipelines<pipeline::SnakePipeline<M>>>()
            .init_resource::<resources::SnakeMeshInstances>();
    }

    fn finish(&self, app: &mut App) {
        app.sub_app_mut(RenderApp)
            .init_resource::<pipeline::SnakePipeline<M>>();
    }
}
