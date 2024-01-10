mod asset_loader;
mod field;
mod input;
mod player;
mod plugins;
mod scene;
mod states;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            scene::ScenePlugin,
            input::InputPlugin,
            player::PlayerPlugin,
            field::FieldPlugin,
            asset_loader::AssetLoaderPlugin,
        ))
        .add_state::<states::GameState>()
        .run();
}
