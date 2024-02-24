use bevy::{
    ecs::entity::EntityHashMap,
    pbr::MeshTransforms,
    prelude::*,
    render::render_resource::{BindGroup, Buffer, ShaderType},
};
#[derive(Default, Resource, Deref, DerefMut)]
pub struct SnakeMeshInstances(EntityHashMap<SnakeMeshInstance>);

#[derive(ShaderType, Copy, Clone, Debug, PartialEq, Reflect, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct SnakeMeshUniforms {
    pub grid_size: Vec3,
    _padding0: u32,
    pub grid_origin: Vec3,
    _padding1: u32,
    pub sphere_origin: Vec3,
    pub sphere_radius: f32,
}

impl SnakeMeshUniforms {
    pub fn new(
        grid_size: Vec3,
        grid_origin: Vec3,
        sphere_origin: Vec3,
        sphere_radius: f32,
    ) -> Self {
        Self {
            grid_size,
            _padding0: 0,
            grid_origin,
            _padding1: 0,
            sphere_origin,
            sphere_radius,
        }
    }
}

pub struct SnakeMeshInstance {
    pub fake_mesh_asset: AssetId<Mesh>,
    pub uniforms: SnakeMeshUniforms,
    pub uniform_buffer: Option<Buffer>,
    pub vertex_buffer: Option<Buffer>,
    pub index_buffer: Option<Buffer>,
    pub cell_buffer: Option<Buffer>,
    pub atomics_buffer: Option<Buffer>,
    pub indirect_buffer: Option<Buffer>,
    pub compute_bind_group: Option<BindGroup>,
    pub vertex_count: usize,
    pub transforms: MeshTransforms,
}
