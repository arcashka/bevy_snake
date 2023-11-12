mod common_types;
mod field_plugin;
mod player_plugin;

use field_plugin::FieldPlugin;
use player_plugin::PlayerPlugin;

use bevy::prelude::*;

fn main() {
    let field_plugin = FieldPlugin::new(IVec2 { x: 100, y: 100 }, Vec2 { x: 0.0, y: 0.0 });
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, field_plugin))
        .run();
}
