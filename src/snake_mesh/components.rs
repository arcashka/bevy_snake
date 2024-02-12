use bevy::{
    pbr::MeshTransforms,
    prelude::*,
    render::render_resource::{BindGroup, Buffer},
};

#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct SnakeMesh {
    pub size: f32,
    pub fake_mesh_asset: AssetId<Mesh>,
}

pub struct SnakeMeshInstance {
    pub fake_mesh_asset: AssetId<Mesh>,
    pub size: f32,
    pub buffer: Option<Buffer>,
    pub compute_bind_group: Option<BindGroup>,
    pub vertex_count: usize,
    pub transforms: MeshTransforms,
}
