use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct TiledMaterialExtension {
    #[uniform(100)]
    field_size: IVec2,

    #[texture(101)]
    #[sampler(102)]
    tile_color: Handle<Image>,
}

impl TiledMaterialExtension {
    pub fn new(size: IVec2, tile_color: Handle<Image>) -> Self {
        Self {
            field_size: size,
            tile_color,
        }
    }
}

impl MaterialExtension for TiledMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        "shaders/tiled_material_extension.wgsl".into()
    }
}

pub struct TiledMaterialPlugin;
impl Plugin for TiledMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, TiledMaterialExtension>,
        >::default());
    }
}
