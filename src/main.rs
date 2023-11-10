mod field_plugin;

use field_plugin::FieldPlugin;
//mod player;

use bevy::prelude::*;

fn main() {
    let field_plugin = FieldPlugin::new(IVec2 { x: 10, y: 10 }, Vec2 { x: 0.0, y: 0.0 });
    App::new()
        .add_plugins((
            DefaultPlugins,
            //player::PlayerLogic,
            field_plugin,
        ))
        .run();
}
