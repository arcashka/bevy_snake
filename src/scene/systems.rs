use bevy::{prelude::*, render::camera::ScalingMode};

pub fn setup(mut commands: Commands) {
    commands.spawn(DirectionalLightBundle {
        transform: Transform::from_xyz(0.0, 100.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        directional_light: DirectionalLight {
            shadows_enabled: true,
            illuminance: 20000.0,
            ..default()
        },
        ..default()
    });
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(0.0, 30.0, 0.0).looking_at(Vec3::ZERO, Vec3::Z),
        projection: Projection::Orthographic(OrthographicProjection {
            scaling_mode: ScalingMode::AutoMin {
                min_width: 20.0,
                min_height: 20.0,
            },
            ..default()
        }),
        ..default()
    });
}
