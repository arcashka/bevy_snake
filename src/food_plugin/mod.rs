use crate::common_types::Cell;
use crate::field_plugin::{Field, FieldId};

use bevy::prelude::*;

#[derive(Resource, Default)]
struct FoodSpawnTimer(Timer);

pub struct FoodPlugin;
impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_food)
            .add_systems(FixedUpdate, (spawn_food, position_food));
    }
}

fn setup_food(mut commands: Commands) {
    commands.insert_resource(FoodSpawnTimer(Timer::from_seconds(
        5.0,
        TimerMode::Repeating,
    )));
}

#[derive(Component)]
struct Food;

fn spawn_food(mut commands: Commands, time: Res<Time>, mut timer: ResMut<FoodSpawnTimer>) {
    if timer.0.tick(time.delta()).just_finished() {
        commands.spawn((
            Food,
            Cell::new(2, 3),
            SpriteBundle {
                transform: Transform {
                    scale: Vec3::new(20.0, 20.0, 0.0),
                    ..default()
                },
                sprite: Sprite {
                    color: Color::rgb(0.0, 0.73, 0.85),
                    ..default()
                },
                ..default()
            },
            FieldId(0),
        ));
    }
}

fn position_food(
    mut food_query: Query<(&mut Transform, &Cell, &FieldId), With<Food>>,
    field_query: Query<(&Field, &FieldId)>,
) {
    for (mut transform, cell, food_field_id) in food_query.iter_mut() {
        for (field, field_id) in field_query.iter() {
            if food_field_id != field_id {
                continue;
            }
            transform.translation = field.translation_of_position(cell).extend(1.0);
        }
    }
}
