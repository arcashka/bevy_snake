use bevy::{
    ecs::entity::EntityHashMap,
    pbr::MeshTransforms,
    prelude::*,
    render::render_resource::{BindGroup, Buffer},
};

#[derive(Default, Resource, Deref, DerefMut)]
pub struct SnakeMeshInstances(EntityHashMap<SnakeMeshInstance>);

pub struct SnakeMeshInstance {
    pub fake_mesh_asset: AssetId<Mesh>,
    pub size: f32,
    pub vertex_buffer: Option<Buffer>,
    pub compute_bind_group: Option<BindGroup>,
    pub vertex_count: usize,
    pub transforms: MeshTransforms,
}
