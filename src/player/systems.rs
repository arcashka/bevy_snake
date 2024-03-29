use std::f32::consts::PI;

use super::components::{
    BodyInfo, DistancePassed, Fragment, PreviousHeadPosition, PreviousHeadPositions, TurnDirection,
    Turning, TurningValue,
};
use super::components::{Direction, Player, Speed, TurnSpeed};
use super::events::MovedOntoNewCellEvent;
use super::resources::PlayerStartSetting;

use crate::asset_loader::{AssetsStorage, SceneAssets};
use crate::field::{Cell, Field};
use crate::input::TurnRequestsBuffer;

use bevy::prelude::*;

pub fn setup(
    mut commands: Commands,
    start_settings: Res<PlayerStartSetting>,
    field: Res<Field>,
    assets_storage: Res<AssetsStorage>,
) {
    info!("snake setup");
    let cell_coordinates = field.translation_of_cell(&start_settings.cell);
    let head_translation = Vec3::new(cell_coordinates.x, 0.5, cell_coordinates.y);
    let turn_moment = 0.0;
    let scaled_speed = start_settings.speed * field.cell_size();
    let cell_part_for_turn = 1.0 - (turn_moment * 2.0);
    let turn_speed = scaled_speed * 2.0 / cell_part_for_turn;

    let scaling = 0.4;
    let start_transform =
        Transform::from_translation(head_translation).with_scale(Vec3::splat(scaling));

    let meshes = [
        assets_storage.handles[&SceneAssets::SnakeHead7].clone(),
        assets_storage.handles[&SceneAssets::SnakeHead6].clone(),
        assets_storage.handles[&SceneAssets::SnakeHead5].clone(),
        assets_storage.handles[&SceneAssets::SnakeHead4].clone(),
        assets_storage.handles[&SceneAssets::SnakeHead3].clone(),
        assets_storage.handles[&SceneAssets::SnakeHead2].clone(),
        assets_storage.handles[&SceneAssets::SnakeHead1].clone(),
    ];

    let mut body_list = Vec::<Entity>::new();
    for fragment_part in 0..50 {
        let scene = if fragment_part < meshes.len() {
            meshes[fragment_part].clone()
        } else {
            assets_storage.handles[&SceneAssets::SnakeMainBody].clone()
        };
        let id = commands
            .spawn((
                SceneBundle {
                    scene,
                    // visibility: Visibility::Hidden,
                    transform: start_transform,
                    ..default()
                },
                DistancePassed(0.0),
                Fragment(fragment_part as u32),
            ))
            .id();
        body_list.push(id);
    }
    commands.spawn((
        SceneBundle {
            scene: assets_storage.handles[&SceneAssets::SnakeHead8].clone(),
            transform: start_transform,
            ..default()
        },
        Player,
        Speed(start_settings.speed),
        start_settings.direction,
        start_settings.cell,
        TurnSpeed(turn_speed),
        Turning(None),
        PreviousHeadPositions(vec![PreviousHeadPosition {
            transform: Transform::from_translation(head_translation),
            distance_passed: DistancePassed(0.0),
        }]),
        DistancePassed(0.0),
        BodyInfo {
            body: body_list,
            first_gap: start_settings.gap,
            gap: start_settings.gap,
        },
    ));
}

pub fn move_head(
    time: Res<Time>,
    mut transform_query: Query<
        (
            &mut Transform,
            &mut Turning,
            &mut Direction,
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
        mut direction,
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
                *direction = Direction::closest_from_rotation(&transform.rotation);
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
    mut turning_query: Query<(&mut Turning, &Transform, &Direction), With<Player>>,
    mut input: ResMut<TurnRequestsBuffer>,
    mut new_cell_events: EventReader<MovedOntoNewCellEvent>,
    // field: Res<Field>,
) {
    for (mut turning, _, direction) in turning_query.iter_mut() {
        if turning.0.is_some() {
            continue;
        }
        // let cell_local_translation = field.cell_local_translation(transform.translation.xz());
        // let cell_size = field.cell_size();
        // let cell_progress = match direction {
        //     Direction::Left => (cell_size - cell_local_translation.x) / cell_size,
        //     Direction::Right => cell_local_translation.x / cell_size,
        //     Direction::Up => cell_local_translation.y / cell_size,
        //     Direction::Down => (cell_size - cell_local_translation.y) / cell_size,
        // };

        for _ in new_cell_events.read() {
            if let Some(turn_request) = input.pop() {
                if let Some(new_direction) =
                    TurnDirection::from_turn_request(*direction, turn_request)
                {
                    turning.0 = Some(TurningValue {
                        direction: new_direction,
                        progress: 0.0,
                    });
                }
            }
        }
    }
}

pub fn check_if_on_new_cell(
    mut player_query: Query<(&mut Cell, &Transform), With<Player>>,
    mut events: EventWriter<MovedOntoNewCellEvent>,
    field: Res<Field>,
) {
    for (mut cell, transform) in player_query.iter_mut() {
        let current_cell = field.cell(transform.translation.xz());
        if current_cell != *cell {
            *cell = current_cell;
            events.send(MovedOntoNewCellEvent {});
        }
    }
}
