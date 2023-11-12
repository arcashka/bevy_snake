mod input_plugin;
mod position;

use crate::common_types::Position;
use crate::field_plugin::{Field, FieldId, HighlightComponent};

use position::Direction;

use input_plugin::InputPlugin;

use bevy::prelude::*;

use self::input_plugin::ChangeDirectionRequested;

#[derive(Component, Clone)]
struct Player;

#[derive(Resource, Clone)]
struct PlayerSettings {
    starting_position: Position,
}

#[derive(Component)]
struct NeedsPositioning;

fn setup(mut commands: Commands, settings: Res<PlayerSettings>) {
    let position = settings.starting_position.to_owned();
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
        NeedsPositioning,
        position,
    ));
}

type PlayerWhichNeedsPositioning = (With<NeedsPositioning>, With<Player>);
fn startup_positioning(
    mut commands: Commands,
    mut player_query: Query<
        (Entity, &Position, &FieldId, &mut Transform),
        PlayerWhichNeedsPositioning,
    >,
    field_query: Query<(&Field, &FieldId)>,
) {
    for (entity, position, player_field_id, mut transform) in player_query.iter_mut() {
        for (field, field_id) in field_query.iter() {
            if player_field_id == field_id {
                transform.translation = field.translation_of_position(position).extend(1.0);
                commands.entity(entity).remove::<NeedsPositioning>();
            }
        }
    }
}

fn player_movement(
    time: Res<Time>,
    mut player_query: Query<(&mut Transform, &Direction), With<Player>>,
) {
    for (mut transform, direction) in player_query.iter_mut() {
        let step = 50.0 * time.delta_seconds();
        match direction {
            Direction::Left => transform.translation.x -= step,
            Direction::Right => transform.translation.x += step,
            Direction::Up => transform.translation.y += step,
            Direction::Down => transform.translation.y -= step,
        }
    }
}

fn check_if_on_new_cell(
    mut player_query: Query<(&Transform, &mut Position, &FieldId), With<Player>>,
    mut field_query: Query<(&Field, &mut HighlightComponent, &FieldId)>,
) {
    for (transform, mut position, player_field_id) in player_query.iter_mut() {
        for (field, mut highlight, field_id) in field_query.iter_mut() {
            if field_id == player_field_id {
                let new_position = field.position_of_translation(transform.translation.xy());
                if new_position != *position {
                    *position = new_position;
                }
                highlight.clear_highlight();
                highlight.highlight(*position);
            }
        }
    }
}

fn handle_turns(
    mut player_query: Query<(&mut Transform, &mut Direction, &Position, &FieldId), With<Player>>,
    field_query: Query<(&Field, &FieldId)>,
    mut turn_requested_events: ResMut<Events<ChangeDirectionRequested>>,
) {
    for (mut transform, mut direction, position, player_field_id) in player_query.iter_mut() {
        for (field, field_id) in field_query.iter() {
            if field_id == player_field_id {
                let center = field.translation_of_position(position);
                if center.distance(transform.translation.xy()) < 0.5 {
                    for event in turn_requested_events.drain() {
                        *direction = event.new_direction;
                        transform.translation = field.translation_of_position(position).extend(1.0);
                    }
                }
            }
        }
    }
}

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputPlugin)
            .insert_resource(PlayerSettings {
                starting_position: Position::new(0, 0),
            })
            .add_systems(Startup, setup)
            .add_systems(
                FixedUpdate,
                (
                    startup_positioning,
                    player_movement.after(startup_positioning),
                    check_if_on_new_cell.after(player_movement),
                    handle_turns,
                ),
            );
    }
}
