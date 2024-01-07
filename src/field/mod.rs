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
            .add_systems(Startup, setup);
    }
}
