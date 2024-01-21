use bevy::{pbr::MaterialBindGroupId, prelude::*, render::render_resource::Buffer};

#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct SnakeMesh {
    pub size: f32,
}

#[derive(Component)]
pub struct SnakeMeshMarker;

#[derive(Component, Clone, PartialEq)]
pub struct SnakeMeshInstance {
    pub size: f32,
    pub material_bind_group_id: MaterialBindGroupId,
}

#[derive(Component)]
pub struct SnakeMeshBuffer {
    pub buffer: Buffer,
    pub length: usize,
}
