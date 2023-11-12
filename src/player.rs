use bevy::prelude::*;

use crate::field_plugin::Field;

#[derive(Clone, Copy)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Component, Clone)]
struct Player {
    direction: Direction,
}

#[derive(Resource, Clone)]
struct PlayerSettings {
    starting_position: Position,
}

#[derive(Resource)]
struct TickTimer(Timer);

#[derive(Resource)]
struct SpawnTimer(Timer);

fn setup(mut commands: Commands, settings: Res<PlayerSettings>, grid: Res<GridSettings>) {
    let position = settings.starting_position.to_owned();
    let translation = position.translation(&grid);
    commands.spawn(Camera2dBundle::default());
    commands.spawn((
        SpriteBundle {
            transform: Transform {
                scale: Vec3::new(20.0, 20.0, 0.0),
                translation: Vec3::new(translation.x, translation.y, 1.0),
                ..default()
            },
            sprite: Sprite {
                color: Color::rgb(1.0, 0.73, 0.85),
                ..default()
            },
            ..default()
        },
        Player {
            direction: Direction::Right,
        },
        position,
    ));
}

fn player_movement(
    time: Res<Time>,
    timer: Res<TickTimer>,
    grid: Res<GridSettings>,
    mut query: Query<(&mut Transform, &Player)>,
) {
    let dt = time.delta_seconds() / timer.0.duration().as_secs_f32();
    for (mut transform, player) in query.iter_mut() {
        let step = grid.cell_size * dt;
        match player.direction {
            Direction::Left => transform.translation.x -= step,
            Direction::Right => transform.translation.x += step,
            Direction::Up => transform.translation.y += step,
            Direction::Down => transform.translation.y -= step,
        }
    }
}

#[derive(Event)]
struct MovedOntoNewCell {
    entity: Entity,
    new_position: Position,
}
fn check_if_on_new_cell(
    grid: Res<GridSettings>,
    query: Query<(Entity, &Player, &Position, &Transform)>,
    mut moved_onto_new_cell_events: EventWriter<MovedOntoNewCell>,
) {
    for (entity, player, position, transform) in query.iter() {
        let new_position = position.single_step_into(player.direction, &grid);
        info!("current position: x: {}, y: {}", position.i, position.j);
        info!("new position: x: {}, y: {}", new_position.i, new_position.j);
        info!(
            "current translation: x: {}, y: {}",
            transform.translation.x, transform.translation.y
        );
        let new_translation = new_position.translation(&grid);
        info!(
            "new position translation: x: {}, y: {}",
            new_translation.x, new_translation.y
        );

        if new_position.matches(transform.translation.xy(), &grid) {
            info!("new position: x: {}, y: {}", new_position.i, new_position.j);
            moved_onto_new_cell_events.send(MovedOntoNewCell {
                entity,
                new_position,
            })
        }
    }
}

fn new_cell_event_listener(
    mut new_cell_events: EventReader<MovedOntoNewCell>,
    mut input_events: EventReader<ChangeDirection>,
    mut query: Query<(&mut Position, &mut Player, Entity)>,
) {
    for event_new_cell in new_cell_events.read() {
        for (mut position, _, entity) in query.iter_mut() {
            if event_new_cell.entity == entity {
                *position = event_new_cell.new_position;
            }
        }
        for event_input in input_events.read() {
            if event_input.entity == event_new_cell.entity {
                for (_, mut player, entity) in query.iter_mut() {
                    if event_input.entity == entity {
                        player.direction = event_input.new_direction;
                    }
                }
            }
        }
    }
}

pub struct PlayerLogic;
impl Plugin for PlayerLogic {
    fn build(&self, app: &mut App) {
        app.insert_resource(TickTimer(Timer::from_seconds(3.0, TimerMode::Repeating)))
            .insert_resource(PlayerSettings {
                starting_position: Position { i: 0, j: 0 },
            })
            .add_event::<MovedOntoNewCell>()
            .add_event::<ChangeDirection>()
            .add_systems(Startup, setup)
            .add_systems(Update, handle_input)
            .add_systems(
                FixedUpdate,
                (
                    player_movement,
                    check_if_on_new_cell,
                    new_cell_event_listener,
                ),
            );
    }
}
