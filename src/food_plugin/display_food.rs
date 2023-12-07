use super::{Food, FoodType};
use crate::field_plugin::{Cell, Field, FieldId};

use bevy::prelude::*;
use lazy_static::lazy_static;
use std::collections::HashMap;

#[derive(Component, Deref, Clone)]
struct AnimationIndices {
    indices: Vec<usize>,
}

#[derive(Component, Deref, DerefMut)]
struct AnimationIndex(usize);

#[derive(Component, Deref, DerefMut)]
struct AnimationTimer(Timer);

struct FoodAsset {
    texture: Handle<TextureAtlas>,
    texture_size: Vec2,
    indices: AnimationIndices,
}

#[derive(Resource)]
struct FoodStorage {
    map: HashMap<FoodType, FoodAsset>,
}

struct TextureInfo {
    filename: &'static str,
    grid_size: Vec2,
    columns: usize,
    rows: usize,
    animation_indices: Vec<usize>,
}

lazy_static! {
    static ref TEXTURE_INFO_MAP: HashMap<FoodType, TextureInfo> = {
        let mut m = HashMap::new();
        m.insert(
            FoodType::Banana,
            TextureInfo {
                filename: "banana.png",
                grid_size: Vec2::new(153.125, 232.0),
                columns: 8,
                rows: 1,
                animation_indices: vec![0, 1, 2, 3, 4, 5],
            },
        );
        m.insert(
            FoodType::Strawberry,
            TextureInfo {
                filename: "strawbery.png",
                grid_size: Vec2::new(64.0, 85.0),
                columns: 1,
                rows: 1,
                animation_indices: vec![0],
            },
        );
        m
    };
}

fn get_food_asset(
    food: &FoodType,
    image_assets: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
) -> FoodAsset {
    let texture_info = TEXTURE_INFO_MAP.get(food).unwrap();
    let texture_handle = image_assets.load(texture_info.filename);
    let texture_atlas = TextureAtlas::from_grid(
        texture_handle,
        texture_info.grid_size,
        texture_info.columns,
        texture_info.rows,
        None,
        None,
    );
    let texture_size = texture_atlas.size;
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    FoodAsset {
        texture: texture_atlas_handle.clone(),
        texture_size,
        indices: AnimationIndices {
            indices: texture_info.animation_indices.clone(),
        },
    }
}

type FoodWithoutSprite = (With<Food>, Without<TextureAtlasSprite>);
fn draw_food(
    mut commands: Commands,
    query: Query<(Entity, &FoodType, &Cell, &FieldId), FoodWithoutSprite>,
    field_query: Query<(Entity, &FieldId, &Field)>,
    image_assets: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut food_storage: ResMut<FoodStorage>,
) {
    for (entity, food_type, cell, food_field_id) in query.iter() {
        let food_asset = match food_storage.map.get(food_type) {
            Some(food_asset) => food_asset,
            None => {
                let asset = get_food_asset(food_type, &image_assets, &mut texture_atlases);
                food_storage.map.insert(*food_type, asset);
                food_storage.map.get(food_type).unwrap()
            }
        };

        let first_index = food_asset.indices.indices[0];
        for (field_entity, field_id, field) in field_query.iter() {
            if food_field_id != field_id {
                continue;
            }
            let translation = field.translation(cell);
            info!("Adding food at {:?}", translation);
            let scale_factor =
                (1.0 / food_asset.texture_size.x).min(1.0 / food_asset.texture_size.y);
            let sprite_sheet = SpriteSheetBundle {
                texture_atlas: food_asset.texture.clone(),
                sprite: TextureAtlasSprite::new(first_index),
                transform: Transform {
                    translation: translation.extend(1.0),
                    scale: Vec3::splat(scale_factor),
                    ..default()
                },
                ..default()
            };
            let food_entity = if food_asset.indices.len() > 1 {
                commands
                    .entity(entity)
                    .insert((
                        sprite_sheet,
                        food_asset.indices.clone(),
                        AnimationIndex(first_index),
                        AnimationTimer(Timer::from_seconds(0.3, TimerMode::Repeating)),
                    ))
                    .id()
            } else {
                commands.entity(entity).insert(sprite_sheet).id()
            };
            commands.entity(field_entity).push_children(&[food_entity]);
        }
    }
}

fn next_frame(
    time: Res<Time>,
    mut query: Query<
        (
            &AnimationIndices,
            &mut AnimationIndex,
            &mut TextureAtlasSprite,
            &mut AnimationTimer,
        ),
        With<Food>,
    >,
) {
    for (indices, mut current_index, mut sprite, mut timer) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            if current_index.0 == indices.len() - 1 {
                current_index.0 = 0;
            } else {
                current_index.0 += 1;
            }
            sprite.index = indices[current_index.0];
        }
    }
}

pub struct FoodDisplayPlugin;
impl Plugin for FoodDisplayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(FoodStorage {
            map: HashMap::new(),
        })
        .add_systems(FixedUpdate, (draw_food, next_frame));
    }
}
