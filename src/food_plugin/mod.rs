mod display_food;

use crate::field_plugin::{Cell, Field, FieldId};

use display_food::FoodDisplayPlugin;

use rand::{thread_rng, Rng};

use bevy::prelude::*;

#[derive(Resource, Default)]
struct FoodSpawnTimer(Timer);

pub struct FoodPlugin;
impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FoodDisplayPlugin)
            .add_systems(Startup, setup_food)
            .add_systems(FixedUpdate, spawn_food);
    }
}

fn setup_food(mut commands: Commands) {
    commands.insert_resource(FoodSpawnTimer(Timer::from_seconds(
        2.0,
        TimerMode::Repeating,
    )));
}

#[derive(Component)]
struct Food;

#[derive(Component, Clone, Copy, Eq, PartialEq, Hash)]
enum FoodType {
    Banana,
    Strawberry,
}

fn spawn_food(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<FoodSpawnTimer>,
    query: Query<(&Field, &FieldId)>,
) {
    for (field, field_id) in query.iter() {
        if timer.0.tick(time.delta()).just_finished() {
            let mut rng = thread_rng();
            let i = rng.gen_range(0..field.dimensions().x);
            let j = rng.gen_range(0..field.dimensions().y);

            commands.spawn((Food, FoodType::Strawberry, Cell::new(i, j), *field_id));
        }
    }
}
