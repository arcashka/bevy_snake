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
        .insert_resource(field::Field::new(
            IVec2 { x: 10, y: 10 },
            Vec2 { x: 10.0, y: 10.0 },
            Vec2 { x: 0.0, y: 0.0 },
        ))
        .run();
}
