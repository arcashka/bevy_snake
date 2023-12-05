mod field_plugin;
mod food_plugin;
mod player_plugin;
mod system_sets;

use field_plugin::FieldPlugin;
use food_plugin::FoodPlugin;
use player_plugin::PlayerPlugin;
use system_sets::GameSystemSets;

use bevy::prelude::*;

fn main() {
    App::new()
        .configure_sets(
            Update,
            GameSystemSets::PlayerSetup.after(GameSystemSets::FieldSetup),
        )
        .add_plugins((
            DefaultPlugins,
            FieldPlugin::new(IVec2 { x: 20, y: 20 }, Vec2 { x: 0.0, y: 0.0 }),
            FoodPlugin,
            PlayerPlugin,
        ))
        .run();
}
