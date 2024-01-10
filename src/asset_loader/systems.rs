use std::collections::HashMap;

use bevy::asset::LoadState;
use bevy::prelude::*;

use super::resources::{AssetPaths, AssetsStorage, SceneAssets};
use crate::states::GameState;

pub fn start_loading(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    asset_paths: Res<AssetPaths>,
) {
    info!("start loading");
    let mut handles = HashMap::<SceneAssets, Handle<Scene>>::new();
    for (asset, path) in asset_paths.0.iter() {
        info!("trying to load asset: {:?}", path);
        handles.insert(*asset, asset_server.load(path));
    }
    info!("done loading");
    commands.insert_resource(AssetsStorage { handles });
}

pub fn check_assets_ready(
    assets_storage: Res<AssetsStorage>,
    asset_server: Res<AssetServer>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    info!("checking assets");
    for asset in assets_storage.handles.values() {
        if let Some(load_state) = asset_server.get_load_state(asset.id()) {
            if load_state != LoadState::Loaded {
                info!("some asset is not loaded, state: {:?}", load_state);
                return;
            }
        } else {
            info!("failed to get load state for asset: {:?}", asset.id());
            return;
        }
    }
    info!("set next state to InGame");
    next_state.set(GameState::InGame);
}
