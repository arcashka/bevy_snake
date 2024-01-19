use bevy::math::Affine3;
use bevy::prelude::*;
use bevy::render::render_resource::{BindGroup, Buffer};

#[derive(Component, Clone)]
pub struct SnakeBuffers {
    pub index_buffer: Buffer,
    pub vertex_buffer: Buffer,
    pub uniform_buffer: Option<Buffer>,
    pub bind_group: Option<BindGroup>,
}

#[derive(Component, Asset, Reflect, Clone)]
pub struct SnakeMesh {
    pub size: f32,
}

#[derive(Component)]
pub struct RenderSnakeMeshInstance {
    pub size: f32,
    pub transform: Affine3,
}
