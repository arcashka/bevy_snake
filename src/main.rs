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
        .insert_resource(field::Field {
            dimensions: IVec2 { x: 10, y: 10 },
            offset: Vec2 { x: 0.0, y: 0.0 },
            size: Vec2 { x: 10.0, y: 10.0 },
        })
        .run();
}
