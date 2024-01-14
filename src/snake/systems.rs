use bevy::{prelude::*, render::camera::ScalingMode};

use super::components::Snake;

pub fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Box::default())),
        material: materials.add(Color::GREEN.into()),
        transform: Transform::from_xyz(0.0, 0.0, 0.0),
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(5.0, 5.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
        projection: Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: 10.0,
                min_height: 10.0,
            },
            ..default()
        }),
        ..default()
    });
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(-100.0, 100.0, -100.0).looking_at(Vec3::ZERO, Vec3::Z),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 20000.0,
            ..default()
        },
        ..default()
    });

    commands.spawn(Snake { size: 1.0 });
}
