use bevy::prelude::*;
use bevy::render::render_resource::{
    BindGroupEntry, BufferDescriptor, BufferInitDescriptor, BufferUsages,
};
use bevy::render::renderer::RenderDevice;
use bevy::render::Extract;

use super::components::{RenderSnakeMeshInstance, SnakeBuffers};
use super::resources::{SnakeBindGroup, SnakePipeline};
use super::SnakeMesh;

pub fn extract_snake_meshes(
    mut commands: Commands,
    query: Extract<Query<(Entity, &ViewVisibility, &GlobalTransform, &SnakeMesh)>>,
) {
    let mut batch = Vec::new();
    for (entity, visibility, transform, mesh) in query.iter() {
        if visibility == &ViewVisibility::HIDDEN {
            continue;
        }
        batch.push(RenderSnakeMeshInstance {
            size: mesh.size,
            transform: (&transform.affine()).into(),
        });
    }
    commands.spawn_batch(batch);
}

pub fn prepare_bind_group(
    mut snake_buffers: Query<&mut SnakeBuffers>,
    pipeline: Res<SnakePipeline>,
    render_device: Res<RenderDevice>,
) {
    for mut snake_buffers in snake_buffers.iter_mut() {
        let bind_group = render_device.create_bind_group(
            Some("Compute bind group"),
            &pipeline.bind_group_layout,
            &[
                BindGroupEntry {
                    binding: 0,
                    resource: snake_buffers.index_buffer.as_entire_binding(),
                },
                BindGroupEntry {
                    binding: 1,
                    resource: snake_buffers.vertex_buffer.as_entire_binding(),
                },
            ],
        );
        snake_buffers.bind_group = Some(bind_group);
        info!("inserted");
    }
}

pub fn create_uniform_buffers(
    mut snake_settings: Query<(&SnakeMesh, &mut SnakeBuffers)>,
    render_device: Res<RenderDevice>,
) {
    for (snake, mut buffers) in snake_settings.iter_mut() {
        let uniform_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
            label: Some("uniform buffer for compute"),
            usage: BufferUsages::UNIFORM,
            #[allow(clippy::unnecessary_cast)]
            contents: &(snake.size as f32).to_ne_bytes(),
        });
        buffers.uniform_buffer = Some(uniform_buffer);
    }
}
