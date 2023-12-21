use bevy::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let model: Handle<Scene> = asset_server.load("models/snake.gltf#Scene0");
    info!("model: {:?}", model);

    commands.spawn(SceneBundle {
        scene: model,
        transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(1.0)),
        ..default()
    });
}
