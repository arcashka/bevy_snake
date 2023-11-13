mod common_types;
mod field_plugin;
mod food_plugin;
mod player_plugin;

use field_plugin::FieldPlugin;
use food_plugin::FoodPlugin;
use player_plugin::PlayerPlugin;

use bevy::prelude::*;

fn main() {
    let field_plugin = FieldPlugin::new(IVec2 { x: 20, y: 20 }, Vec2 { x: 0.0, y: 0.0 });
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, field_plugin, FoodPlugin))
        .run();
}
