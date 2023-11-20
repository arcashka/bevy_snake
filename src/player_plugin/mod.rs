mod input_plugin;
mod position;

use bevy::prelude::*;

use crate::field_plugin::{Cell, Field, FieldId, HighlightComponent};
use crate::food_plugin::Interactable;
use input_plugin::{ChangeDirectionRequested, InputPlugin};
use position::Direction;

#[derive(Component, Clone)]
struct Player;

#[derive(Resource, Clone)]
struct PlayerSettings {
    starting_position: Cell,
}

#[derive(Component, Clone, Copy, Debug)]
struct ProgressTowardsNextCell(f32);

#[derive(Component, Clone, Copy, Deref, DerefMut)]
struct ProgressAlreadyReported(bool);

fn setup(mut commands: Commands, settings: Res<PlayerSettings>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                scale: Vec3::new(30.0, 30.0, 0.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 0.73, 0.85),
                ..default()
            },
            ..default()
        },
        Player,
        Direction::Right,
        FieldId(0),
        settings.starting_position,
        ProgressTowardsNextCell(0.0),
        ProgressAlreadyReported(false),
    ));
}

fn position_on_field(
    mut player_query: Query<
        (
            &Cell,
            &ProgressTowardsNextCell,
            &Direction,
            &FieldId,
            &mut Transform,
        ),
        With<Player>,
    >,
    field_query: Query<(&Field, &FieldId)>,
) {
    for (cell, progress, direction, player_field_id, mut transform) in player_query.iter_mut() {
        for (field, field_id) in field_query.iter() {
            if player_field_id != field_id {
                continue;
            }
            let base_translation = field.translation_of_position(cell).extend(1.0);
            let next_cell = field.single_step_into(cell, direction);
            let next_cell_translation = field.translation_of_position(&next_cell).extend(1.0);
            transform.translation =
                base_translation * (1.0 - progress.0) + next_cell_translation * progress.0;
        }
    }
}

fn move_forward(
    time: Res<Time>,
    mut query: Query<
        (
            Entity,
            &mut ProgressTowardsNextCell,
            &mut Cell,
            &mut Direction,
            &mut Transform,
            &mut ProgressAlreadyReported,
            &FieldId,
        ),
        With<Player>,
    >,
    field_query: Query<(&Field, &FieldId)>,
    mut turn_requested_events: ResMut<Events<ChangeDirectionRequested>>,
    mut stepped_on_new_cell_events: EventWriter<StepedOnNewCell>,
) {
    for (
        entity,
        mut progress,
        mut cell,
        mut direction,
        mut transform,
        mut progress_already_reported,
        player_field_id,
    ) in query.iter_mut()
    {
        let step = time.delta_seconds() * 5.0;
        progress.0 += step;
        for (field, field_id) in field_query.iter() {
            if field_id != player_field_id {
                continue;
            }
            let next_cell = field.single_step_into(&cell, &direction);
            if progress.0 >= 0.5 && !progress_already_reported.0 {
                stepped_on_new_cell_events.send(StepedOnNewCell {
                    entity,
                    cell: next_cell,
                });
                progress_already_reported.0 = true;
            }
            if progress.0 < 1.0 {
                continue;
            }
            // new cell - new progress
            *cell = next_cell;
            progress.0 = 0.0;
            progress_already_reported.0 = false;
            for event in turn_requested_events.drain() {
                *direction = event.new_direction;
                transform.translation = field.translation_of_position(&cell).extend(1.0);
            }
        }
    }
}

fn highlight_cell(
    query: Query<&FieldId, With<Player>>,
    mut field_query: Query<(&mut HighlightComponent, &FieldId)>,
    mut events: EventReader<StepedOnNewCell>,
) {
    for player_field_id in query.iter() {
        for (mut highlight, field_id) in field_query.iter_mut() {
            if field_id != player_field_id {
                continue;
            }
            for event in events.read() {
                let next_cell = event.cell;
                if highlight.is_highlighted(&next_cell) {
                    continue;
                }
                highlight.clear_highlight();
                highlight.highlight(next_cell);
            }
        }
    }
}

fn check_collision(
    player_query: Query<Entity, With<Player>>,
    other_query: Query<(Entity, &Cell), With<Interactable>>,
    mut stepped_on_new_cell_events: EventReader<StepedOnNewCell>,
    mut collision_events: EventWriter<CollisionHappened>,
) {
    for new_cell_event in stepped_on_new_cell_events.read() {
        for player in player_query.iter() {
            if player != new_cell_event.entity {
                continue;
            }
            for (other, cell) in other_query.iter() {
                if new_cell_event.cell != *cell {
                    continue;
                }
                collision_events.send(CollisionHappened { player, other })
            }
        }
    }
}

#[derive(Event)]
struct StepedOnNewCell {
    entity: Entity,
    cell: Cell,
}

#[derive(Event)]
struct CollisionHappened {
    player: Entity,
    other: Entity,
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputPlugin)
            .insert_resource(PlayerSettings {
                starting_position: Cell::new(0, 0),
            })
            .add_event::<StepedOnNewCell>()
            .add_event::<CollisionHappened>()
            .add_systems(Startup, setup)
            .add_systems(
                FixedUpdate,
                (
                    move_forward,
                    position_on_field.after(move_forward),
                    highlight_cell.after(move_forward),
                    check_collision.after(move_forward),
                ),
            );
    }
}
