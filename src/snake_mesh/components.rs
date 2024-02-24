use bevy::prelude::*;

#[derive(Component, Copy, Clone, Debug, PartialEq, Reflect)]
pub struct PolygonizationSettings {
    pub grid_size: Vec3,
    pub grid_origin: Vec3,
}

#[derive(Component, Copy, Clone, Debug, PartialEq, Reflect)]
pub struct SnakeMesh {
    pub radius: f32,
    pub center: Vec3,
    pub fake_mesh_asset: AssetId<Mesh>,
}
