mod field_plugin;
mod food_plugin;
mod player_plugin;

use field_plugin::FieldPlugin;
use food_plugin::FoodPlugin;
use player_plugin::PlayerPlugin;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            FoodPlugin,
            PlayerPlugin,
            FieldPlugin::new(IVec2 { x: 20, y: 20 }, Vec2 { x: 0.0, y: 0.0 }),
        ))
        .run();
}
