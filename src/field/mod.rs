mod components;
mod resources;
mod systems;

use bevy::prelude::*;

pub use components::{Cell, Field, FieldId};
pub use resources::FieldSettings;

use systems::setup;

use crate::plugins::TiledMaterialPlugin;
use crate::system_sets::GameSystemSets;

pub struct FieldPlugin {
    pub settings: FieldSettings,
}

impl Plugin for FieldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(TiledMaterialPlugin)
            .insert_resource(self.settings)
            .add_systems(Startup, setup.in_set(GameSystemSets::FieldSetup));
    }
}
