use std::f32::consts::PI;

use super::components::{
    BodyInfo, DistanceFromStart, Fragment, PreviousTransforms, TurnDirection, Turning, TurningValue,
};
use super::helpers::Direction;
use super::{Player, Speed, TurnSpeed};

use crate::input::TurnRequestsBuffer;

use bevy::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let model: Handle<Scene> = asset_server.load("models/snake_head.gltf#Scene0");
    let body_model: Handle<Scene> = asset_server.load("models/snake_body.gltf#Scene0");
    let default_transform = Transform::from_xyz(0.0, 0.0, 0.0).with_scale(Vec3::splat(1.0));

    let mut body_list = Vec::<Entity>::new();
    for fragment_part in 0..100 {
        let id = commands
            .spawn((
                SceneBundle {
                    scene: body_model.to_owned(),
                    transform: default_transform,
                    ..default()
                },
                DistanceFromStart(0.0),
                Fragment(fragment_part),
            ))
            .id();
        body_list.push(id);
    }
    commands.spawn((
        SceneBundle {
            scene: model,
            transform: default_transform,
            ..default()
        },
        Player,
        Speed(5.0),
        TurnSpeed(10.0),
        Turning(None),
        PreviousTransforms {
            body: vec![default_transform],
        },
        DistanceFromStart(0.0),
        BodyInfo {
            body: body_list,
            first_gap: 2.0,
            gap: 0.5,
        },
    ));
}

pub fn move_head(
    time: Res<Time>,
    mut transform_query: Query<
        (
            &mut Transform,
            &mut Turning,
            &mut PreviousTransforms,
            &Speed,
            &TurnSpeed,
        ),
        With<Player>,
    >,
) {
    for (mut transform, mut turning, mut previous_transforms, speed, turn_speed) in
        transform_query.iter_mut()
    {
        if turning.0.is_some() {
            let turning_unwrapped = turning.0.as_mut().unwrap();
            let turn_delta = turn_speed.0 * time.delta_seconds();
            turning_unwrapped.progress += turn_delta;
            if turning_unwrapped.progress >= PI / 2.0 {
                let direction = Direction::closest_from_rotation(&transform.rotation);
                transform.rotation = direction.quaternion();
                turning.0 = None;
            } else {
                transform.rotation *=
                    Quat::from_axis_angle(Vec3::Y, turn_delta * turning_unwrapped.direction.sign());
            }
        }
        let forward = transform.rotation * Vec3::Z;
        transform.translation += forward * time.delta_seconds() * speed.0;
        previous_transforms.body.push(transform.to_owned());
    }
}

pub fn move_body(
    mut fragment_query: Query<&mut Transform, (With<Fragment>, Without<Player>)>,
    previous_transforms_query: Query<
        (&PreviousTransforms, &BodyInfo, &Transform),
        (With<Player>, Without<Fragment>),
    >,
) {
    for (previous_transforms, body_info, head_transform) in previous_transforms_query.iter() {
        let head = head_transform.translation;
        let mut next_fragment_distance = body_info.first_gap;
        let mut fragment_pointer: usize = 0;
        let last = previous_transforms.body.len() - 1;
        for fragment_id in body_info.body.iter() {
            let mut fragment_transform = fragment_query.get_mut(*fragment_id).unwrap();
            for i in fragment_pointer..last {
                let old_body_transform = previous_transforms.body[last - i];
                if old_body_transform.translation.distance(head) >= next_fragment_distance {
                    fragment_pointer = i;
                    break;
                }
            }
            *fragment_transform = previous_transforms.body[last - fragment_pointer];
            next_fragment_distance += body_info.gap;
        }
    }
}

pub fn handle_input(
    mut turning_query: Query<(&mut Turning, &Transform), With<Player>>,
    mut input: ResMut<TurnRequestsBuffer>,
) {
    for (mut turning, transform) in turning_query.iter_mut() {
        if let Some(new_direction) = input.pop() {
            let direction = Direction::closest_from_rotation(&transform.rotation);
            if let Some(direction) = TurnDirection::from_turn_request(direction, new_direction) {
                turning.0 = Some(TurningValue {
                    direction,
                    progress: 0.0,
                });
            }
        }
    }
}
