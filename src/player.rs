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

#[derive(Component, Clone, Copy)]
struct Position {
    i: i32,
    j: i32,
}

impl Position {
    fn step_into(&self, direction: Direction, step: i32, grid: &GridSettings) -> Self {
        let mut i = self.i;
        let mut j = self.j;
        match direction {
            Direction::Left => i -= step,
            Direction::Right => i += step,
            Direction::Up => j -= step,
            Direction::Down => j += step,
        };
        fn wrap(x: i32, max: i32) -> i32 {
            if x < 0 {
                max + x
            } else if x >= max {
                x - max
            } else {
                x
            }
        }
        Self {
            i: wrap(i, grid.num_cols),
            j: wrap(j, grid.num_rows),
        }
    }

    fn single_step_into(&self, direction: Direction, grid: &GridSettings) -> Self {
        self.step_into(direction, 1, grid)
    }

    fn translation(&self, grid: &GridSettings) -> Vec2 {
        let h = grid.cell_size * grid.num_rows as f32;
        let w = grid.cell_size * grid.num_cols as f32;
        Vec2::new(
            self.i as f32 * grid.cell_size - w / 2.0,
            self.j as f32 * grid.cell_size - h / 2.0,
        )
    }

    fn matches(&self, translation: Vec2, grid: &GridSettings) -> bool {
        let this_translation = self.translation(grid);
        translation.distance(this_translation) < 0.01
    }
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

#[derive(Event)]
struct ChangeDirection {
    entity: Entity,
    new_direction: Direction,
}

fn handle_input(
    key: Res<Input<KeyCode>>,
    query: Query<Entity, With<Player>>,
    mut change_direction_events: EventWriter<ChangeDirection>,
) {
    for entity in query.iter() {
        let new_direction = if key.pressed(KeyCode::Left) {
            Some(Direction::Left)
        } else if key.pressed(KeyCode::Right) {
            Some(Direction::Right)
        } else if key.pressed(KeyCode::Up) {
            Some(Direction::Up)
        } else if key.pressed(KeyCode::Down) {
            Some(Direction::Down)
        } else {
            None
        };

        if let Some(direction) = new_direction {
            change_direction_events.send(ChangeDirection {
                entity,
                new_direction: direction,
            });
        }
    }
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
