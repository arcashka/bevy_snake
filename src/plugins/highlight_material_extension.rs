use bevy::{
    pbr::{ExtendedMaterial, MaterialExtension},
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
};

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct HighlightMaterialExtension {
    #[uniform(100)]
    field_size: IVec2,
    #[uniform(101)]
    highlight_list_length: i32,
    #[storage(102)]
    highlight_list: Vec<IVec2>,
}

impl HighlightMaterialExtension {
    pub fn new(size: IVec2) -> Self {
        Self {
            field_size: size,
            highlight_list_length: 0,
            highlight_list: Vec::new(),
        }
    }

    pub fn set_highlighted(&mut self, highlighted: Vec<IVec2>) {
        self.highlight_list = highlighted;
        self.highlight_list_length = self.highlight_list.len() as i32;
    }
}

impl MaterialExtension for HighlightMaterialExtension {
    fn fragment_shader() -> ShaderRef {
        "shaders/highlight_material_extension.wgsl".into()
    }
}

pub struct HighlightMaterialPlugin;
impl Plugin for HighlightMaterialPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<
            ExtendedMaterial<StandardMaterial, HighlightMaterialExtension>,
        >::default());
    }
}
