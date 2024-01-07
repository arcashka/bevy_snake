mod components;
mod resources;
mod systems;

use bevy::prelude::*;

pub use components::Cell;
pub use resources::Field;

use systems::setup;

use crate::plugins::TiledMaterialPlugin;

pub struct FieldPlugin;

impl Plugin for FieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TiledMaterialPlugin)
            .add_systems(Startup, setup)
            .insert_resource(Field::new(
                IVec2 { x: 10, y: 10 },
                Vec2 { x: 10.0, y: 10.0 },
                Vec2 { x: 0.0, y: 0.0 },
            ));
    }
}
