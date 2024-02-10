use bevy::{
    pbr::{MaterialBindGroupId, MeshTransforms},
    prelude::*,
    render::render_resource::Buffer,
};

#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct SnakeMesh {
    pub size: f32,
    pub fake_mesh_asset: AssetId<Mesh>,
}

pub struct SnakeMeshInstance {
    pub fake_mesh_asset: AssetId<Mesh>,
    pub size: f32,
    pub material_bind_group_id: MaterialBindGroupId,
    pub buffer: Option<Buffer>,
    pub buffer_length: usize,
    pub transforms: MeshTransforms,
}
