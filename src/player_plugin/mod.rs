mod input_plugin;
mod position;
mod sprites;

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
    speed: f32,
}

#[derive(Component, Clone, Copy, Debug)]
struct ProgressTowardsNextCell(f32);

#[derive(Component, Clone, Copy, Deref, DerefMut, PartialEq, Debug)]
pub struct PlayerId(i32);

#[derive(Component)]
struct Speed(f32);

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

#[derive(Component, PartialEq, Debug)]
enum FragmentType {
    Head,
    Tail,
    Body,
    HeadAndTail,
}

impl FragmentType {
    fn is_head(&self) -> bool {
        matches!(self, FragmentType::Head | FragmentType::HeadAndTail)
    }

    fn is_tail(&self) -> bool {
        matches!(self, FragmentType::Tail | FragmentType::HeadAndTail)
    }
}

fn setup(
    mut commands: Commands,
    field_query: Query<Entity, With<Field>>,
    settings: Res<PlayerSettings>,
    snake_sprite_sheet: Res<sprites::SnakeSpriteSheet>,
) {
    for field_entity in field_query.iter() {
        commands.spawn((
            Player,
            FieldId(0),
            PlayerId(0),
            ProgressTowardsNextCell(0.0),
            Speed(settings.speed),
        ));
        let entity = commands
            .spawn((
                Fragment,
                FragmentType::HeadAndTail,
                snake_sprite_sheet.0.clone(),
                PlayerId(0),
                settings.starting_position,
                FragmentNumber(0),
                Direction::Right,
            ))
            .id();
        commands.entity(field_entity).push_children(&[entity]);
    }
}

fn position_fragments(
    mut fragments_query: Query<(&PlayerId, &Cell, &Direction, &mut Transform), With<Fragment>>,
    player_query: Query<(&PlayerId, &FieldId, &ProgressTowardsNextCell), With<Player>>,
    field_query: Query<(&Field, &FieldId)>,
) {
    for (player_id, player_field_id, progress) in player_query.iter() {
        for (field, field_id) in field_query.iter() {
            if player_field_id != field_id {
                continue;
            }
            for (fragment_player_id, cell, direction, mut transform) in fragments_query.iter_mut() {
                if player_id != fragment_player_id {
                    continue;
                }
                let next_cell = field.single_step_into(cell, direction);
                let base_translation = field.translation(cell);
                let next_cell_translation = field.translation(&next_cell);
                transform.translation = (base_translation * (1.0 - progress.0)
                    + next_cell_translation * progress.0)
                    .extend(1.0);
            }
        }
    }
}

fn make_step(
    time: Res<Time>,
    mut query: Query<(&PlayerId, &Speed, &mut ProgressTowardsNextCell), With<Player>>,
    mut stepped_on_new_cell_events: EventWriter<ShouldMoveOntoNextCellEvent>,
) {
    for (player_id, speed, mut progress) in query.iter_mut() {
        let step = time.delta_seconds() * speed.0;
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
    mut fragments_query: Query<(&PlayerId, &Direction, &FragmentType, &mut Cell), With<Fragment>>,
    player_query: Query<(&PlayerId, &FieldId), With<Player>>,
    field_query: Query<(&Field, &FieldId)>,
    mut should_move_onto_new_cell_events: EventReader<ShouldMoveOntoNextCellEvent>,
    mut moved_onto_new_cell_events: EventWriter<MovedOntoNextCellEvent>,
) {
    for event in should_move_onto_new_cell_events.read() {
        for (player_id, player_field_id) in player_query.iter() {
            if event.player_id != *player_id {
                continue;
            }
            for (field, field_id) in field_query.iter() {
                if player_field_id != field_id {
                    continue;
                }
                for (fragment_player_id, direction, fragment_type, mut cell) in
                    fragments_query.iter_mut()
                {
                    if player_id != fragment_player_id {
                        continue;
                    }

                    *cell = field.single_step_into(&cell, direction);
                    if fragment_type.is_head() {
                        moved_onto_new_cell_events.send(MovedOntoNextCellEvent {
                            player_id: *player_id,
                            cell: *cell,
                        })
                    }
                }
            }
        }
    }
}

fn update_direction(
    player_query: Query<(&PlayerId, &FieldId), With<Player>>,
    field_query: Query<&FieldId, With<Field>>,
    mut fragments_query: Query<
        (&PlayerId, &FragmentNumber, &FragmentType, &mut Direction),
        With<Fragment>,
    >,
    mut turn_requests_buffer: ResMut<TurnRequestsBuffer>,
    mut moved_onto_new_cell_events: EventReader<MovedOntoNextCellEvent>,
) {
    for event in moved_onto_new_cell_events.read() {
        for field_id in field_query.iter() {
            for (player_id, player_field_id) in player_query.iter() {
                if event.player_id != *player_id {
                    continue;
                }
                if field_id != player_field_id {
                    continue;
                }
                let mut fragments = fragments_query
                    .iter_mut()
                    .filter(|(fragment_player_id, _, _, _)| *fragment_player_id == player_id)
                    .collect::<Vec<_>>();

                fragments.sort_by(|l, r| {
                    let l_number = l.1;
                    let r_number = r.1;
                    r_number.cmp(l_number)
                });
                let fragments_len = fragments.len();
                for i in 0..fragments_len - 1 {
                    let (_, _, _, next_fragment_direction) = &fragments[i + 1];
                    let next_fragment_direction = **next_fragment_direction;
                    let (_, _, _, ref mut direction) = fragments[i];
                    **direction = next_fragment_direction;
                }
                let (_, _, _, ref mut head_direction) = fragments[fragments_len - 1];
                let turn_request = turn_requests_buffer.pop();
                let new_direction = if let Some(turn_request) = turn_request {
                    turn_request
                } else {
                    **head_direction
                };
                **head_direction = new_direction;
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
    player_query: Query<(&PlayerId, &FieldId), With<Player>>,
    field_query: Query<(Entity, &Field, &FieldId)>,
    mut fragments_query: Query<
        (
            &PlayerId,
            &FragmentNumber,
            &Cell,
            &mut FragmentType,
            &Direction,
        ),
        With<Fragment>,
    >,
    mut collision_events: EventReader<CollisionEvent>,
    mut commands: Commands,
    snake_sprite_sheet: Res<sprites::SnakeSpriteSheet>,
) {
    for collision_event in collision_events.read() {
        for (player_id, player_field_id) in player_query.iter() {
            if collision_event.player != *player_id {
                continue;
            }
            for (field_entity, field, field_id) in field_query.iter() {
                if player_field_id != field_id {
                    continue;
                }
                for (fragment_player_id, fragment_number, cell, mut fragment_type, direction) in
                    fragments_query.iter_mut()
                {
                    if collision_event.player != *fragment_player_id {
                        continue;
                    }

                    if !fragment_type.is_tail() {
                        continue;
                    }

                    let new_fragment_entity = commands
                        .spawn((
                            Fragment,
                            FragmentType::Tail,
                            snake_sprite_sheet.0.clone(),
                            *player_id,
                            field.single_step_into(cell, &direction.opposite()),
                            FragmentNumber(fragment_number.0 + 1),
                            *direction,
                        ))
                        .id();
                    commands
                        .entity(field_entity)
                        .push_children(&[new_fragment_entity]);

                    if *fragment_type == FragmentType::HeadAndTail {
                        *fragment_type = FragmentType::Head;
                    } else {
                        *fragment_type = FragmentType::Body;
                    }
                }
            }
        }
    }
}

pub struct PlayerPlugin {
    pub speed: f32,
}

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputPlugin)
            .insert_resource(PlayerSettings {
                starting_position: Cell::new(0, 0),
                speed: self.speed,
            })
            .insert_resource(sprites::SnakeSpriteSheet(SpriteSheetBundle::default()))
            .add_event::<ShouldMoveOntoNextCellEvent>()
            .add_event::<MovedOntoNextCellEvent>()
            .add_event::<CollisionEvent>()
            .add_systems(
                Startup,
                (
                    sprites::init_snake_sprite_sheet.in_set(GameSystemSets::PlayerSetup),
                    setup
                        .in_set(GameSystemSets::PlayerSetup)
                        .after(sprites::init_snake_sprite_sheet),
                ),
            )
            .add_systems(
                FixedUpdate,
                (
                    make_step,
                    move_onto_new_cell.after(make_step),
                    check_collision.after(move_onto_new_cell),
                    grow_snake_on_feeding
                        .after(check_collision)
                        .before(update_direction),
                    update_direction.after(move_onto_new_cell),
                    position_fragments.after(move_onto_new_cell),
                    sprites::update_fragment_sprites,
                ),
            );
    }
}
