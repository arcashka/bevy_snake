use bevy::prelude::*;

#[derive(Component, Copy, Clone, Debug, PartialEq)]
pub struct SnakeMesh {
    pub size: f32,
    pub fake_mesh_asset: AssetId<Mesh>,
}
