mod resources;
mod systems;

use std::collections::HashMap;

use bevy::prelude::*;

use crate::states::GameState;

use resources::AssetPaths;
pub use resources::{AssetsStorage, SceneAssets};

pub struct AssetLoaderPlugin;
impl Plugin for AssetLoaderPlugin {
    fn build(&self, app: &mut App) {
        let mut paths = HashMap::new();
        paths.insert(
            SceneAssets::SnakeMainBody,
            "models/main_body.glb#Scene0".to_string(),
        );
        paths.insert(
            SceneAssets::SnakeHead1,
            "models/head_01.glb#Scene0".to_string(),
        );
        paths.insert(
            SceneAssets::SnakeHead2,
            "models/head_02.glb#Scene0".to_string(),
        );
        paths.insert(
            SceneAssets::SnakeHead3,
            "models/head_03.glb#Scene0".to_string(),
        );
        paths.insert(
            SceneAssets::SnakeHead4,
            "models/head_04.glb#Scene0".to_string(),
        );
        paths.insert(
            SceneAssets::SnakeHead5,
            "models/head_05.glb#Scene0".to_string(),
        );
        paths.insert(
            SceneAssets::SnakeHead6,
            "models/head_06.glb#Scene0".to_string(),
        );
        paths.insert(
            SceneAssets::SnakeHead7,
            "models/head_07.glb#Scene0".to_string(),
        );
        paths.insert(
            SceneAssets::SnakeHead8,
            "models/head_08.glb#Scene0".to_string(),
        );
        app.insert_resource(AssetPaths(paths));
        app.add_systems(OnEnter(GameState::Loading), systems::start_loading)
            .add_systems(
                FixedUpdate,
                systems::check_assets_ready.run_if(in_state(GameState::Loading)),
            );
    }
}
