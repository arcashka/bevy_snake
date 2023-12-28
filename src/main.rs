mod field;
mod input;
mod player;
mod plugins;
mod scene;
mod system_sets;

use system_sets::GameSystemSets;

use bevy::prelude::*;

fn main() {
    App::new()
        .configure_sets(
            Startup,
            GameSystemSets::PlayerSetup.after(GameSystemSets::FieldSetup),
        )
        .add_plugins((
            scene::ScenePlugin,
            input::InputPlugin,
            player::PlayerPlugin,
            field::FieldPlugin {
                settings: field::FieldSettings {
                    dimensions: IVec2 { x: 10, y: 10 },
                    offset: Vec2 { x: 0.0, y: 0.0 },
                    size: Vec2 { x: 10.0, y: 10.0 },
                },
            },
        ))
        .run();
}
