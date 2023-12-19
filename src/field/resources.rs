use bevy::prelude::*;

#[derive(Resource, Clone, Copy)]
pub struct FieldSettings {
    pub dimensions: IVec2,
    pub offset: Vec2,
}
