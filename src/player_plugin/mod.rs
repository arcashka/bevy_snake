mod input_plugin;
mod position;

use crate::field_plugin::{Cell, Field, FieldId, HighlightComponent};

use position::Direction;

use input_plugin::InputPlugin;

use bevy::prelude::*;

use self::input_plugin::ChangeDirectionRequested;

#[derive(Component, Clone)]
struct Player;

#[derive(Resource, Clone)]
struct PlayerSettings {
    starting_position: Cell,
}

#[derive(Component, Clone, Copy, Debug)]
struct ProgressTowardsNextCell(f32);

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
            &mut ProgressTowardsNextCell,
            &mut Cell,
            &mut Direction,
            &mut Transform,
            &FieldId,
        ),
        With<Player>,
    >,
    field_query: Query<(&Field, &FieldId)>,
    mut turn_requested_events: ResMut<Events<ChangeDirectionRequested>>,
) {
    for (mut progress, mut cell, mut direction, mut transform, player_field_id) in query.iter_mut()
    {
        let step = time.delta_seconds() * 5.0;
        progress.0 += step;
        if progress.0 < 1.0 {
            continue;
        }
        progress.0 = 0.0;
        for (field, field_id) in field_query.iter() {
            if field_id != player_field_id {
                continue;
            }
            *cell = field.single_step_into(&cell, &direction);
            for event in turn_requested_events.drain() {
                *direction = event.new_direction;
                transform.translation = field.translation_of_position(&cell).extend(1.0);
            }
        }
    }
}

fn highlight_cell(
    query: Query<(&ProgressTowardsNextCell, &Cell, &Direction, &FieldId), With<Player>>,
    mut field_query: Query<(&Field, &mut HighlightComponent, &FieldId)>,
) {
    for (progress, cell, direction, player_field_id) in query.iter() {
        if progress.0 < 0.5 {
            continue;
        }
        for (field, mut highlight, field_id) in field_query.iter_mut() {
            if field_id != player_field_id {
                continue;
            }
            let next_cell = field.single_step_into(cell, direction);
            if highlight.is_highlighted(&next_cell) {
                continue;
            }
            highlight.clear_highlight();
            highlight.highlight(next_cell);
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
            .add_systems(Startup, setup)
            .add_systems(
                FixedUpdate,
                (
                    move_forward,
                    position_on_field.after(move_forward),
                    highlight_cell.after(move_forward),
                ),
            );
    }
}
