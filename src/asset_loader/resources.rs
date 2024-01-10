use std::collections::HashMap;

use bevy::prelude::*;

#[derive(Hash, Eq, PartialEq, Clone, Copy, Debug)]
pub enum SceneAssets {
    SnakeMainBody,
    SnakeHead1,
    SnakeHead2,
    SnakeHead3,
    SnakeHead4,
    SnakeHead5,
    SnakeHead6,
    SnakeHead7,
    SnakeHead8,
}

#[derive(Resource)]
pub struct AssetsStorage {
    pub handles: HashMap<SceneAssets, Handle<Scene>>,
}

#[derive(Resource)]
pub struct AssetPaths(pub HashMap<SceneAssets, String>);
