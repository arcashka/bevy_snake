mod input_plugin;
mod position;

use bevy::prelude::*;

use crate::field_plugin::{Cell, Field, FieldId};
use crate::food_plugin::Interactable;
use crate::system_sets::GameSystemSets;

use input_plugin::{InputPlugin, TurnRequestsBuffer};
use position::Direction;

#[derive(Component, Clone)]
struct Player;

#[derive(Component, Clone)]
struct Fragment;

#[derive(Resource, Clone)]
struct PlayerSettings {
    starting_position: Cell,
}

#[derive(Component, Clone, Copy, Debug)]
struct ProgressTowardsNextCell(f32);

#[derive(Component, Clone, Copy, Deref, DerefMut, PartialEq, Debug)]
pub struct PlayerId(i32);

#[derive(Component, Clone, Copy, Deref, DerefMut, Eq, PartialEq, PartialOrd, Ord)]
struct FragmentNumber(usize);

#[derive(Event)]
struct ShouldMoveOntoNextCellEvent {
    player_id: PlayerId,
}

#[derive(Event)]
struct MovedOntoNextCellEvent {
    player_id: PlayerId,
    cell: Cell,
}

#[derive(Event, Debug)]
pub struct CollisionEvent {
    pub player: PlayerId,
    pub other: Entity,
}

const SNAKE_HEAD_INDEX: usize = 0;

fn setup(
    mut commands: Commands,
    field_query: Query<Entity, With<Field>>,
    settings: Res<PlayerSettings>,
) {
    for field_entity in field_query.iter() {
        commands.spawn((
            Player,
            FieldId(0),
            PlayerId(0),
            Direction::Right,
            ProgressTowardsNextCell(0.0),
        ));
        let entity = commands
            .spawn((
                Fragment,
                SpriteBundle {
                    transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
                    sprite: Sprite {
                        color: Color::rgb(0.21, 0.73, 0.21),
                        custom_size: Some(Vec2::new(1.0, 1.0)),
                        ..default()
                    },
                    ..default()
                },
                PlayerId(0),
                settings.starting_position,
                FragmentNumber(0),
            ))
            .id();
        commands.entity(field_entity).push_children(&[entity]);
    }
}

fn position_fragments(
    mut fragments_query: Query<(&PlayerId, &FragmentNumber, &Cell, &mut Transform), With<Fragment>>,
    player_query: Query<(&PlayerId, &Direction, &FieldId, &ProgressTowardsNextCell), With<Player>>,
    field_query: Query<(&Field, &FieldId)>,
) {
    for (player_id, direction, player_field_id, progress) in player_query.iter() {
        for (field, field_id) in field_query.iter() {
            if player_field_id != field_id {
                continue;
            }
            let mut fragments = fragments_query
                .iter_mut()
                .filter(|(fragment_player_id, _, _, _)| *fragment_player_id == player_id)
                .collect::<Vec<_>>();

            fragments.sort_by(|l, r| {
                let l_number = l.1;
                let r_number = r.1;
                l_number.cmp(r_number)
            });
            for i in 0..fragments.len() {
                let (_, _, cell, _) = fragments[i];
                let next_cell = if i > SNAKE_HEAD_INDEX {
                    let (_, _, next_fragment_cell, _) = fragments[i - 1];
                    *next_fragment_cell
                } else {
                    field.single_step_into(cell, direction)
                };
                let base_translation = Vec2 {
                    x: cell.i() as f32 - field.dimensions.x as f32 / 2.0,
                    y: cell.j() as f32 - field.dimensions.y as f32 / 2.0,
                };
                let next_cell_translation = Vec2 {
                    x: next_cell.i() as f32 - field.dimensions.x as f32 / 2.0,
                    y: next_cell.j() as f32 - field.dimensions.y as f32 / 2.0,
                };
                let (_, _, _, ref mut transform) = &mut fragments[i];
                transform.translation = (base_translation * (1.0 - progress.0)
                    + next_cell_translation * progress.0)
                    .extend(1.0);
            }
        }
    }
}

fn make_step(
    time: Res<Time>,
    mut query: Query<(&PlayerId, &mut ProgressTowardsNextCell), With<Player>>,
    mut stepped_on_new_cell_events: EventWriter<ShouldMoveOntoNextCellEvent>,
) {
    for (player_id, mut progress) in query.iter_mut() {
        let step = time.delta_seconds() * 5.0;
        progress.0 += step;
        if progress.0 >= 1.0 {
            stepped_on_new_cell_events.send(ShouldMoveOntoNextCellEvent {
                player_id: *player_id,
            });
            progress.0 = 0.0;
        }
    }
}

fn move_onto_new_cell(
    mut fragments_query: Query<(&PlayerId, &FragmentNumber, &mut Cell), With<Fragment>>,
    player_query: Query<(&PlayerId, &FieldId, &Direction), With<Player>>,
    field_query: Query<(&Field, &FieldId)>,
    mut should_move_onto_new_cell_events: EventReader<ShouldMoveOntoNextCellEvent>,
    mut moved_onto_new_cell_events: EventWriter<MovedOntoNextCellEvent>,
) {
    for event in should_move_onto_new_cell_events.read() {
        for (player_id, player_field_id, direction) in player_query.iter() {
            if event.player_id != *player_id {
                continue;
            }
            for (field, field_id) in field_query.iter() {
                if player_field_id != field_id {
                    continue;
                }
                let mut fragments = fragments_query
                    .iter_mut()
                    .filter(|(fragment_player_id, _, _)| *fragment_player_id == player_id)
                    .collect::<Vec<_>>();

                fragments.sort_by(|l, r| {
                    let l_number = l.1;
                    let r_number = r.1;
                    r_number.cmp(l_number)
                });
                for i in 0..fragments.len() {
                    let (_, number, cell) = &mut fragments[i];
                    let next_cell = if number.0 == SNAKE_HEAD_INDEX {
                        field.single_step_into(cell, direction)
                    } else {
                        let (_, _, next_fragment_cell) = &fragments[i + 1];
                        **next_fragment_cell
                    };
                    let (_, number, ref mut cell) = fragments[i];
                    **cell = next_cell;
                    if number.0 == 0 {
                        moved_onto_new_cell_events.send(MovedOntoNextCellEvent {
                            player_id: *player_id,
                            cell: next_cell,
                        })
                    }
                }
            }
        }
    }
}

fn apply_input(
    mut player_query: Query<(&PlayerId, &mut Direction), With<Player>>,
    mut moved_onto_next_cell_events: EventReader<MovedOntoNextCellEvent>,
    mut turn_requests_buffer: ResMut<TurnRequestsBuffer>,
) {
    for new_cell_event in moved_onto_next_cell_events.read() {
        for (player_id, mut direction) in player_query.iter_mut() {
            if new_cell_event.player_id != *player_id {
                continue;
            }
            let turn_request = turn_requests_buffer.pop();
            if let Some(turn_request) = turn_request {
                *direction = turn_request;
            }
        }
    }
}

fn check_collision(
    player_query: Query<&PlayerId, With<Player>>,
    other_query: Query<(Entity, &Cell), With<Interactable>>,
    mut moved_onto_next_cell_events: EventReader<MovedOntoNextCellEvent>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    for new_cell_event in moved_onto_next_cell_events.read() {
        for player_id in player_query.iter() {
            if *player_id != new_cell_event.player_id {
                continue;
            }
            for (other, cell) in other_query.iter() {
                if new_cell_event.cell != *cell {
                    continue;
                }
                collision_events.send(CollisionEvent {
                    player: *player_id,
                    other,
                })
            }
        }
    }
}

fn grow_snake_on_feeding(
    player_query: Query<(&PlayerId, &FieldId, &Direction), With<Player>>,
    fragments_query: Query<(&PlayerId, &FragmentNumber, &Cell), With<Fragment>>,
    field_query: Query<(Entity, &Field, &FieldId)>,
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
) {
    for collision_event in collision_events.read() {
        let mut fragments = fragments_query
            .iter()
            .filter(|(fragment_player_id, _, _)| **fragment_player_id == collision_event.player)
            .collect::<Vec<_>>();

        fragments.sort_by(|l, r| {
            let l_number = l.1;
            let r_number = r.1;
            r_number.cmp(l_number)
        });
        for (player_id, player_field_id, direction) in player_query.iter() {
            if collision_event.player != *player_id {
                continue;
            }
            for (field_entity, field, field_id) in field_query.iter() {
                if player_field_id != field_id {
                    continue;
                }
                let tail_direction = if fragments.len() > 1 {
                    let last = fragments[0].2;
                    let pre_last = fragments[1].2;
                    field.direction(last, pre_last)
                } else {
                    *direction
                };
                let tail = fragments[0].2;
                let tail_number = fragments[0].1;
                let new_number = FragmentNumber(tail_number.0 + 1);
                let new_cell = field.single_step_into(tail, &tail_direction.opposite());
                let new_fragment_entity = commands
                    .spawn((
                        Fragment,
                        SpriteBundle {
                            transform: Transform {
                                // hide behind the field, correct position will be set in position_fragments
                                translation: Vec3::new(0.0, 0.0, -1.0),
                                ..default()
                            },
                            sprite: Sprite {
                                color: Color::rgb(1.0, 0.73, 0.85),
                                ..default()
                            },
                            ..default()
                        },
                        *player_id,
                        new_cell,
                        new_number,
                    ))
                    .id();
                commands
                    .entity(field_entity)
                    .push_children(&[new_fragment_entity]);
            }
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputPlugin)
            .insert_resource(PlayerSettings {
                starting_position: Cell::new(0, 0),
            })
            .add_event::<ShouldMoveOntoNextCellEvent>()
            .add_event::<MovedOntoNextCellEvent>()
            .add_event::<CollisionEvent>()
            .add_systems(Startup, setup.in_set(GameSystemSets::PlayerSetup))
            .add_systems(
                FixedUpdate,
                (
                    make_step,
                    move_onto_new_cell,
                    check_collision,
                    grow_snake_on_feeding,
                    position_fragments,
                    apply_input,
                )
                    .chain(),
            );
    }
}
