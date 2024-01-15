use bevy::prelude::*;
use bevy::render::render_resource::Buffer;

#[derive(Component)]
pub struct SnakeMeshBuffers {
    pub index_buffer: Buffer,
    pub vertex_buffer: Buffer,
}

#[derive(Component, Asset, Reflect, Clone)]
pub struct SnakeMesh {
    pub size: f32,
}
