use std::f32::consts::PI;

use super::Player;
use bevy::{prelude::*, render::mesh::skinning::SkinnedMesh};

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let model: Handle<Scene> = asset_server.load("models/snake_head.gltf#Scene0");
    info!("model: {:?}", model);

    commands.spawn((
        SceneBundle {
            scene: model,
            transform: Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(1.0)),
            ..default()
        },
        Player,
    ));
}

pub fn update_joints(
    time: Res<Time>,
    skinned_meshes: Query<&SkinnedMesh>,
    mut transform_query: Query<&mut Transform>,
) {
    info!("called update_joints");
    for mesh in skinned_meshes.iter() {
        info!("mesh.joints: {}", mesh.joints.len());
        let joint = mesh.joints[0];
        if let Ok(mut joint_transform) = transform_query.get_mut(joint) {
            //joint_transform.translation.y = (time.elapsed_seconds() / 10.0).sin() * 10.0;
        }
        let joint = mesh.joints[1];
        if let Ok(mut joint_transform) = transform_query.get_mut(joint) {
            joint_transform.translation.y = (time.elapsed_seconds() + PI / 10.0).sin() * 2.0 + 4.0;
            // joint_transform.rotate(Quat::from_rotation_z(time.delta_seconds().sin()));
        }
        let joint = mesh.joints[2];
        if let Ok(mut joint_transform) = transform_query.get_mut(joint) {
            //joint_transform.translation.x = time.elapsed_seconds().sin();
            info!("rotation: {}", time.delta_seconds().sin());
            joint_transform.rotation.z = time.elapsed_seconds().sin() / 2.0;
            joint_transform.translation.x = (time.elapsed_seconds() + PI / 2.0).sin() * 2.0;
            joint_transform.translation.y = (time.elapsed_seconds() + PI / 10.0).sin() * 2.0 + 4.0;
            // joint_transform.rotate(Quat::from_rotation_z(time.delta_seconds().sin()));
            // joint_transform.rotate(Quat::from_rotation_z(time.delta_seconds().sin()));
        }
    }
}
