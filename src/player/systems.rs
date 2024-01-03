use std::f32::consts::PI;

use super::components::{
    BodyInfo, DistancePassed, Fragment, PreviousHeadPosition, PreviousHeadPositions, TurnDirection,
    Turning, TurningValue,
};
use super::components::{Player, Speed, TurnSpeed};
use super::events::MovedOntoNewCellEvent;
use super::helpers::Direction;

use crate::field::{Cell, Field, FieldId};
use crate::input::TurnRequestsBuffer;

use bevy::prelude::*;

pub fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let model: Handle<Scene> = asset_server.load("models/snake_head.gltf#Scene0");
    let body_model: Handle<Scene> = asset_server.load("models/snake_body.gltf#Scene0");
    let default_transform = Transform::from_xyz(-0.5, 0.0, -0.5).with_scale(Vec3::splat(0.5));

    let mut body_list = Vec::<Entity>::new();
    for fragment_part in 0..100 {
        let id = commands
            .spawn((
                SceneBundle {
                    scene: body_model.to_owned(),
                    transform: default_transform,
                    ..default()
                },
                DistancePassed(0.0),
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
        Speed(3.0),
        TurnSpeed(6.0),
        Turning(None),
        PreviousHeadPositions(vec![PreviousHeadPosition {
            transform: default_transform,
            distance_passed: DistancePassed(0.0),
        }]),
        DistancePassed(0.0),
        BodyInfo {
            body: body_list,
            first_gap: 0.5,
            gap: 0.2,
        },
        FieldId(0),
        Cell::new(4, 4),
    ));
}

pub fn move_head(
    time: Res<Time>,
    mut transform_query: Query<
        (
            &mut Transform,
            &mut Turning,
            &mut PreviousHeadPositions,
            &mut DistancePassed,
            &Speed,
            &TurnSpeed,
        ),
        With<Player>,
    >,
) {
    for (
        mut transform,
        mut turning,
        mut previous_transforms,
        mut distance_passed,
        speed,
        turn_speed,
    ) in transform_query.iter_mut()
    {
        if let Some(turning_unwrapped) = turning.0.as_mut() {
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
        let distance = time.delta_seconds() * speed.0;
        transform.translation += forward * distance;
        distance_passed.0 += distance;
        previous_transforms.0.push(PreviousHeadPosition {
            transform: transform.to_owned(),
            distance_passed: distance_passed.to_owned(),
        });
    }
}

pub fn move_body(
    mut fragment_query: Query<&mut Transform, (With<Fragment>, Without<Player>)>,
    previous_transforms_query: Query<
        (&PreviousHeadPositions, &BodyInfo, &DistancePassed),
        (With<Player>, Without<Fragment>),
    >,
) {
    for (previous_head_positions, body_info, head_distance_passed) in
        previous_transforms_query.iter()
    {
        let mut next_fragment_distance = body_info.first_gap;
        let mut fragment_pointer: usize = 0;
        let last = previous_head_positions.0.len() - 1;
        for fragment_id in body_info.body.iter() {
            for i in fragment_pointer..last {
                let previous_head = previous_head_positions.0[last - i];
                if head_distance_passed.0 - previous_head.distance_passed.0
                    >= next_fragment_distance
                {
                    fragment_pointer = i;
                    break;
                }
            }
            let mut fragment_transform = fragment_query.get_mut(*fragment_id).unwrap();
            *fragment_transform = previous_head_positions.0[last - fragment_pointer].transform;
            next_fragment_distance += body_info.gap;
        }
    }
}

pub fn handle_input(
    mut turning_query: Query<(&mut Turning, &mut Transform, &FieldId), With<Player>>,
    mut input: ResMut<TurnRequestsBuffer>,
    field_query: Query<(&Field, &FieldId)>,
    mut new_cell_events: EventReader<MovedOntoNewCellEvent>,
) {
    for (mut turning, transform, player_field_id) in turning_query.iter_mut() {
        for (_, field_id) in field_query.iter() {
            if player_field_id != field_id {
                continue;
            }

            for _ in new_cell_events.read() {
                info!("checking for input");
                if let Some(turn_request) = input.pop() {
                    info!("got turn request: {:?}", turn_request);
                    let direction = Direction::closest_from_rotation(&transform.rotation);
                    if let Some(direction) =
                        TurnDirection::from_turn_request(direction, turn_request)
                    {
                        turning.0 = Some(TurningValue {
                            direction,
                            progress: 0.0,
                        });
                    }
                }
            }
        }
    }
}

pub fn check_if_on_new_cell(
    mut player_query: Query<(&mut Cell, &Transform, &FieldId), With<Player>>,
    field_query: Query<(&Field, &FieldId)>,
    mut events: EventWriter<MovedOntoNewCellEvent>,
) {
    for (mut cell, transform, player_field_id) in player_query.iter_mut() {
        for (field, field_id) in field_query.iter() {
            if field_id != player_field_id {
                continue;
            }
            let current_cell = field.cell(transform.translation.xz());
            if current_cell != *cell {
                *cell = current_cell;
                events.send(MovedOntoNewCellEvent {});
            }
        }
    }
}
