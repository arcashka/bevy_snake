use super::components::{TurnDirection, Turning, TurningValue};
use crate::input::{MovementDirection, TurnRequestsBuffer};

use super::{Player, Speed, TurnSpeed};
use bevy::prelude::*;

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
        MovementDirection::Left,
        Speed(1.0),
        TurnSpeed(1.0),
        Turning(None),
    ));
}

pub fn update_head_transform(
    time: Res<Time>,
    mut transform_query: Query<(&mut Transform, &mut Turning, &Speed, &TurnSpeed), With<Player>>,
) {
    for (mut transform, mut turning, speed, turn_speed) in transform_query.iter_mut() {
        if turning.0.is_some() {
            let turning_unwrapped = turning.0.as_mut().unwrap();
            let turn_delta = turn_speed.0 * time.delta_seconds();
            turning_unwrapped.progress += turn_delta;
            if turning_unwrapped.progress >= 90.0 {
                transform.rotation =
                    Quat::from_axis_angle(Vec3::Y, turning_unwrapped.target.degree());
                turning.0 = None;
            } else {
                transform.rotation *=
                    Quat::from_axis_angle(Vec3::Y, turn_delta * turning_unwrapped.direction.sign());
            }
        }
        let forward = transform.rotation * Vec3::Z;
        transform.translation += forward * time.delta_seconds() * speed.0;
    }
}

pub fn handle_input(
    mut turning_query: Query<(&mut Turning, &MovementDirection), With<Player>>,
    mut input: ResMut<TurnRequestsBuffer>,
) {
    for (mut turning, current_direction) in turning_query.iter_mut() {
        if let Some(new_direction) = input.pop() {
            if let Some(turn_direction) =
                TurnDirection::from_turn_request(*current_direction, new_direction)
            {
                turning.0 = Some(TurningValue {
                    direction: turn_direction,
                    target: new_direction,
                    progress: 0.0,
                });
            }
        }
    }
}
