use bevy::prelude::*;
use bevy::render::render_resource::{
    BindGroupEntry, BufferDescriptor, BufferInitDescriptor, BufferUsages,
};
use bevy::render::renderer::RenderDevice;

use super::resources::{SnakeBindGroup, SnakeBuffers, SnakePipeline};

pub fn prepare_bind_group(
    mut commands: Commands,
    pipeline: Res<SnakePipeline>,
    snake_buffers: Res<SnakeBuffers>,
    render_device: Res<RenderDevice>,
) {
    let bind_group = render_device.create_bind_group(
        Some("Compute bind group"),
        &pipeline.bind_group_layout,
        &[
            BindGroupEntry {
                binding: 0,
                resource: snake_buffers.uniform_buffer.as_entire_binding(),
            },
            BindGroupEntry {
                binding: 1,
                resource: snake_buffers.storage_buffer.as_entire_binding(),
            },
        ],
    );
    commands.insert_resource(SnakeBindGroup(bind_group));
}

pub fn create_buffers(mut commands: Commands, render_device: Res<RenderDevice>) {
    let storage_buffer = render_device.create_buffer(&BufferDescriptor {
        size: 1024,
        mapped_at_creation: false,
        label: Some("storage buffer for compute"),
        usage: BufferUsages::STORAGE,
    });
    let uniform_buffer = render_device.create_buffer_with_data(&BufferInitDescriptor {
        label: Some("uniform buffer for compute"),
        usage: BufferUsages::UNIFORM,
        #[allow(clippy::unnecessary_cast)]
        // TODO: Replace 10.0 with size from Snake component
        contents: &(10.0 as f32).to_ne_bytes(),
    });
    commands.insert_resource(SnakeBuffers {
        storage_buffer,
        uniform_buffer,
    });
}
