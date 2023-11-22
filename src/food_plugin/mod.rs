mod display_food;

use crate::field_plugin::{Cell, Field, FieldId};
use crate::player_plugin::CollisionHappened;

use display_food::FoodDisplayPlugin;

use bevy::prelude::*;
use rand::{thread_rng, Rng};

#[derive(Resource, Default)]
struct FoodSpawnTimer(Timer);

pub struct FoodPlugin;
impl Plugin for FoodPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FoodDisplayPlugin)
            .add_systems(Startup, setup_food)
            .add_systems(FixedUpdate, spawn_food)
            .add_systems(Update, handle_collision);
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

#[derive(Component)]
pub struct Interactable;

#[derive(Component, Clone, Copy, Eq, PartialEq, Hash)]
enum FoodType {
    Banana,
    Strawberry,
}

fn handle_collision(
    mut commands: Commands,
    query: Query<Entity, With<Food>>,
    mut events: EventReader<CollisionHappened>,
) {
    info!("CHECKING....");
    for event in events.read() {
        info!("GOT COLLISION EVENT");
        for food in query.iter() {
            if event.other == food {
                info!("Despawning food");
                commands.entity(food).despawn();
            }
        }
    }
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

            commands.spawn((
                Food,
                FoodType::Strawberry,
                Cell::new(i, j),
                *field_id,
                Interactable,
            ));
        }
    }
}
