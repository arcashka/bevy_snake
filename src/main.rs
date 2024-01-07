mod field;
mod input;
mod player;
mod plugins;
mod scene;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            scene::ScenePlugin,
            input::InputPlugin,
            player::PlayerPlugin,
            field::FieldPlugin,
        ))
        .run();
}
