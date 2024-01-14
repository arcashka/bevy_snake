mod asset_loader;
mod field;
mod input;
mod player;
mod plugins;
mod scene;
mod snake;
mod states;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            // scene::ScenePlugin,
            // input::InputPlugin,
            // player::PlayerPlugin,
            // field::FieldPlugin,
            // asset_loader::AssetLoaderPlugin,
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    // uncomment for unthrottled FPS
                    // present_mode: bevy::window::PresentMode::AutoNoVsync,
                    ..default()
                }),
                ..default()
            }),
            snake::SnakePlugin,
        ))
        // .add_state::<states::GameState>()
        .run();
}
