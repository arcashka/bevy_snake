use super::Cell;

use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, ShaderRef},
    sprite::Material2d,
};

#[derive(Component)]
pub struct HighlightComponent {
    highlight_list: Vec<Cell>,
}

impl HighlightComponent {
    pub fn new() -> Self {
        Self {
            highlight_list: Vec::new(),
        }
    }

    pub fn is_highlighted(&self, pos: &Cell) -> bool {
        self.highlight_list.contains(pos)
    }

    pub fn clear_highlight(&mut self) {
        self.highlight_list.clear();
    }

    pub fn highlighted(&self) -> Vec<Cell> {
        self.highlight_list.clone()
    }

    pub fn highlight(&mut self, pos: Cell) {
        self.highlight_list.push(pos);
    }
}

#[derive(AsBindGroup, Debug, Clone, Asset, TypePath)]
pub struct FieldMaterial {
    #[uniform(0)]
    field_size: IVec2,
    #[uniform(1)]
    highlight_list_length: i32,
    #[storage(2)]
    highlight_list: Vec<IVec2>,

    #[texture(3)]
    #[sampler(4)]
    color_texture: Option<Handle<Image>>,
}

impl Material2d for FieldMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/field_material.wgsl".into()
    }
}

impl Material for FieldMaterial {}

impl FieldMaterial {
    pub fn new(size: IVec2, texture: Option<Handle<Image>>) -> Self {
        Self {
            field_size: size,
            highlight_list_length: 0,
            highlight_list: Vec::new(),
            color_texture: texture,
        }
    }

    pub fn set_highlighted(&mut self, highlighted: Vec<Cell>) {
        self.highlight_list = highlighted
            .iter()
            .map(|pos| IVec2::new(pos.i(), pos.j()))
            .collect();
        self.highlight_list_length = self.highlight_list.len() as i32;
    }
}
