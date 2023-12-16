mod input_plugin;
mod position;
mod sprites;

use bevy::pbr::wireframe::{Wireframe, WireframeColor, WireframeConfig, WireframePlugin};
use bevy::prelude::*;
use bevy::render::camera::CameraProjection;
use bevy::render::mesh::{Indices, PrimitiveTopology, VertexAttributeValues};
use bevy::render::primitives::Frustum;
use bevy::sprite::{MaterialMesh2dBundle, Mesh2dHandle};

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

#[derive(Component, PartialEq, Debug, Clone, Copy)]
enum TurnDirection {
    None,
    Left,
    Right,
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

fn turn_direction_from_directions(first: Direction, second: Direction) -> Option<TurnDirection> {
    match first {
        Direction::Up => match second {
            Direction::Left => Some(TurnDirection::Left),
            Direction::Right => Some(TurnDirection::Right),
            _ => None,
        },
        Direction::Down => match second {
            Direction::Left => Some(TurnDirection::Right),
            Direction::Right => Some(TurnDirection::Left),
            _ => None,
        },
        Direction::Left => match second {
            Direction::Up => Some(TurnDirection::Right),
            Direction::Down => Some(TurnDirection::Left),
            _ => None,
        },
        Direction::Right => match second {
            Direction::Up => Some(TurnDirection::Left),
            Direction::Down => Some(TurnDirection::Right),
            _ => None,
        },
    }
}

#[derive(Component, Clone, Copy, Debug)]
struct TurnAngle(f32);

fn setup(
    mut commands: Commands,
    field_query: Query<Entity, With<Field>>,
    settings: Res<PlayerSettings>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    asset_server: Res<AssetServer>,
    // snake_sprite_sheet: Res<sprites::SnakeSpriteSheet>,
) {
    info!("setup player");
    for field_entity in field_query.iter() {
        commands.spawn((
            Player,
            FieldId(0),
            PlayerId(0),
            ProgressTowardsNextCell(0.0),
            Speed(settings.speed),
        ));
        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            vec![
                [-5.0, -5.0, 1.0],
                [5.0, -5.0, 1.0],
                [5.0, 5.0, 1.0],
                [-5.0, 5.0, 1.0],
            ],
        );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_UV_0,
            vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
        );
        mesh.set_indices(Some(Indices::U32(vec![0, 1, 2, 0, 2, 3])));
        let mesh_handle = meshes.add(mesh);
        let texture_handle = asset_server.load("snake_pink_body.png");
        let material_handle = materials.add(ColorMaterial::from(texture_handle));
        let mesh2dbundle = MaterialMesh2dBundle {
            mesh: Mesh2dHandle(mesh_handle),
            material: material_handle,
            ..Default::default()
        };
        let entity = commands
            .spawn((
                Fragment,
                FragmentType::HeadAndTail,
                mesh2dbundle,
                // snake_sprite_sheet.0.clone(),
                PlayerId(0),
                settings.starting_position,
                FragmentNumber(0),
                Direction::Right,
                TurnDirection::None,
                TurnAngle(0.0),
            ))
            .id();
        commands.entity(field_entity).push_children(&[entity]);
    }
}

fn new_setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut color_materials: ResMut<Assets<ColorMaterial>>,
    mut standard_materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    // commands.spawn(Camera2dBundle::default());
    let projection = Projection::Orthographic(OrthographicProjection {
        far: 1000.,
        near: -1000.,
        scale: 2.0,
        ..Default::default()
    });
    let transform = Transform::default();
    let view_projection = projection.get_projection_matrix() * transform.compute_matrix().inverse();
    let frustum = Frustum::from_view_projection_custom_far(
        &view_projection,
        &transform.translation,
        &transform.back(),
        projection.far(),
    );
    commands.spawn(Camera3dBundle {
        projection,
        frustum,
        ..default()
    });

    let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-100.0, -200.0, 1.0],
            [100.0, -200.0, 1.0],
            [100.0, 200.0, 1.0],
            [-100.0, 200.0, 1.0],
        ],
    );
    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![[0.0, 0.0], [1.0, 0.0], [1.0, 1.0], [0.0, 1.0]],
    );

    // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 4]);
    // if let Err(e) = mesh.generate_tangents() {
    //     error!(":( {}", e);
    // }

    mesh.set_indices(Some(Indices::U32(vec![0, 1, 2, 0, 2, 3])));
    let mesh_handle = meshes.add(mesh);

    //let mesh_handle = meshes.add(Mesh::from(shape::Plane::from_size(5.0)));
    // let texture_handle = asset_server.load("snake_pink_body.png");
    // let material_handle = materials.add(ColorMaterial::from(texture_handle));
    let color_material_handle = color_materials.add(Color::ALICE_BLUE.into());
    let mut standard_material: StandardMaterial = Color::WHITE.into();
    let standard_material_handle = standard_materials.add(standard_material);
    // let mesh2dbundle = MaterialMesh2dBundle {
    //     mesh: Mesh2dHandle(mesh_handle),
    //     material: material_handle,
    //     ..Default::default()
    // };
    // let pbr_bundle = PbrBundle {
    //     mesh: mesh_handle,
    //     material: material_handle,
    //     ..default()
    // };

    commands.insert_resource(AmbientLight {
        color: Color::WHITE, // You can change the color as needed
        brightness: 1.0,     // Adjust the brightness as necessary
    });

    commands.spawn((
        PbrBundle {
            mesh: mesh_handle,
            material: standard_material_handle,
            ..default()
        },
        TurnAngle(0.0),
        Fragment,
        Wireframe,
        WireframeColor { color: Color::RED },
    ));
}

fn update_mesh(
    mut mesh_query: Query<(&Handle<Mesh>, &mut TurnAngle), With<Fragment>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut turn_requests_buffer: ResMut<TurnRequestsBuffer>,
) {
    let turn_request = turn_requests_buffer.pop();
    if turn_request.is_none() {
        return;
    }
    for (handle, mut angle) in mesh_query.iter_mut() {
        let mesh = meshes.get_mut(handle).unwrap();
        let mut vertex_count = None;
        for (id, values) in mesh.attributes_mut() {
            if id == Mesh::ATTRIBUTE_POSITION.id {
                if let VertexAttributeValues::Float32x3(positions) = values {
                    angle.0 += 0.02 * std::f32::consts::PI;
                    info!("update_mesh");
                    positions.push([
                        (300.0 * angle.0.cos()) - 200.0,
                        200.0 + (300.0 * angle.0.sin()),
                        1.0,
                    ]);
                    positions.push([
                        (100.0 * angle.0.cos()) - 200.0,
                        200.0 + (100.0 * angle.0.sin()),
                        1.0,
                    ]);
                    vertex_count = Some(positions.len());
                    info!("new positions: {:?}", positions);
                }
            }
            if id == Mesh::ATTRIBUTE_UV_0.id {
                if let VertexAttributeValues::Float32x2(uv) = values {
                    uv.push([0.0, 0.7 + (angle.0 * 0.1)]);
                    uv.push([0.0, 0.7 + (angle.0 * 0.1)]);
                    info!("new uv: {:?}", uv);
                }
            }
        }

        if let Some(vertex_count) = vertex_count {
            let last_idx = vertex_count as u32;
            let extra_triangle_1: Vec<u32> = vec![last_idx - 3, last_idx - 4, last_idx - 2];
            let extra_triangle_2: Vec<u32> = vec![last_idx - 3, last_idx - 2, last_idx - 1];
            let indices = mesh.indices();
            info!("indices: {:?}", indices);
            if let Some(indices) = indices {
                info!("indices: {:?}", indices);
                let new_indices: Vec<u32> = indices
                    .iter()
                    .map(|i| i as u32)
                    .chain(extra_triangle_1.iter().copied())
                    .chain(extra_triangle_2.iter().copied())
                    .collect();
                info!("new indices: {:?}", new_indices);
                mesh.set_indices(Some(Indices::U32(new_indices)));
            }
        }
    }
}

fn position_fragments(
    mut fragments_query: Query<
        (
            &PlayerId,
            &Cell,
            &Direction,
            &FragmentType,
            &TurnDirection,
            &mut Transform,
        ),
        With<Fragment>,
    >,
    player_query: Query<(&PlayerId, &FieldId, &ProgressTowardsNextCell), With<Player>>,
    field_query: Query<(&Field, &FieldId)>,
) {
    for (player_id, player_field_id, progress) in player_query.iter() {
        for (field, field_id) in field_query.iter() {
            if player_field_id != field_id {
                continue;
            }
            for (
                fragment_player_id,
                cell,
                direction,
                fragment_type,
                turn_direction,
                mut transform,
            ) in fragments_query.iter_mut()
            {
                if player_id != fragment_player_id {
                    continue;
                }
                let next_cell = field.single_step_into(cell, direction);
                let base_translation = field.translation(cell);
                let next_cell_translation = field.translation(&next_cell);
                if *turn_direction != TurnDirection::None && *fragment_type == FragmentType::Body {
                    transform.translation = next_cell_translation.extend(1.0);
                } else {
                    transform.translation = (base_translation * (1.0 - progress.0)
                        + next_cell_translation * progress.0)
                        .extend(1.0);
                }
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
                if let Some(turn_request) = turn_request {
                    **head_direction = turn_request;
                }
            }
        }
    }
}

fn update_turns(
    player_query: Query<(&PlayerId, &FieldId), With<Player>>,
    field_query: Query<&FieldId, With<Field>>,
    mut fragments_query: Query<
        (
            &PlayerId,
            &FragmentNumber,
            &FragmentType,
            &Direction,
            &mut TurnDirection,
        ),
        With<Fragment>,
    >,
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
                    .filter(|(fragment_player_id, _, _, _, _)| *fragment_player_id == player_id)
                    .collect::<Vec<_>>();

                fragments.sort_by(|l, r| {
                    let l_number = l.1;
                    let r_number = r.1;
                    r_number.cmp(l_number)
                });
                let fragments_len = fragments.len();
                for i in 0..fragments_len - 1 {
                    let (_, _, _, next_fragment_direction, _) = &fragments[i + 1];
                    let next_fragment_direction = **next_fragment_direction;
                    let (_, _, _, direction, ref mut turn) = fragments[i];
                    if let Some(new_turn) =
                        turn_direction_from_directions(*direction, next_fragment_direction)
                    {
                        **turn = new_turn;
                    } else {
                        **turn = TurnDirection::None;
                    }
                }
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

                    info!("new fragment direction: {:?}", direction);
                    let new_fragment_entity = commands
                        .spawn((
                            Fragment,
                            FragmentType::Tail,
                            snake_sprite_sheet.0.clone(),
                            *player_id,
                            field.single_step_into(cell, &direction.opposite()),
                            FragmentNumber(fragment_number.0 + 1),
                            TurnDirection::None,
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
            .add_plugins(WireframePlugin)
            .insert_resource(WireframeConfig {
                // The global wireframe config enables drawing of wireframes on every mesh,
                // except those with `NoWireframe`. Meshes with `Wireframe` will always have a wireframe,
                // regardless of the global configuration.
                global: true,
                // Controls the default color of all wireframes. Used as the default color for global wireframes.
                // Can be changed per mesh using the `WireframeColor` component.
                default_color: Color::GREEN,
            })
            .insert_resource(PlayerSettings {
                starting_position: Cell::new(0, 0),
                speed: self.speed,
            })
            .insert_resource(sprites::SnakeSpriteSheet(SpriteSheetBundle::default()))
            .add_event::<ShouldMoveOntoNextCellEvent>()
            .add_event::<MovedOntoNextCellEvent>()
            .add_event::<CollisionEvent>()
            .add_systems(Startup, new_setup)
            .add_systems(FixedUpdate, update_mesh);
        // .add_systems(
        //     Startup,
        //     (
        //         sprites::init_snake_sprite_sheet.in_set(GameSystemSets::PlayerSetup),
        //         setup.in_set(GameSystemSets::PlayerSetup),
        //         update_mesh,
        //     )
        //         .chain(),
        // )
        //.add_systems(
        //    FixedUpdate,
        //    update_mesh, //FixedUpdate,
        //                 //(
        //                 //    make_step,
        //                 //    move_onto_new_cell.after(make_step),
        //                 //    check_collision.after(move_onto_new_cell),
        //                 //    grow_snake_on_feeding
        //                 //        .after(check_collision)
        //                 //        .before(update_direction),
        //                 //    update_direction.after(move_onto_new_cell),
        //                 //    update_turns.after(update_direction),
        //                 //    position_fragments.after(move_onto_new_cell),
        //                 //    sprites::update_fragment_sprites.after(update_turns),
        //                 //),
        //);
    }
}
