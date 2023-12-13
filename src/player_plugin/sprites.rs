use bevy::prelude::*;

use super::{Direction, Fragment, FragmentType};

#[derive(Resource)]
pub struct SnakeSpriteSheet(pub SpriteSheetBundle);

fn animation_index(fragment_type: &FragmentType, direction: &Direction) -> usize {
    match fragment_type {
        FragmentType::Body => match direction {
            Direction::Up => 7,
            Direction::Down => 7,
            Direction::Left => 1,
            Direction::Right => 1,
        },
        FragmentType::Head | FragmentType::HeadAndTail => match direction {
            Direction::Up => 3,
            Direction::Down => 9,
            Direction::Left => 8,
            Direction::Right => 4,
        },
        FragmentType::Tail => match direction {
            Direction::Up => 13,
            Direction::Down => 19,
            Direction::Left => 18,
            Direction::Right => 14,
        },
        // FragmentType::Turn => match direction {
        //     Direction::Up => 5,
        //     Direction::Down => 19,
        //     Direction::Left => 2,
        //     Direction::Right => 0,
        // },
    }
}

pub fn update_fragment_sprites(
    mut fragments_query: Query<
        (&FragmentType, &Direction, &mut TextureAtlasSprite),
        (
            With<Fragment>,
            Or<(Changed<FragmentType>, Changed<Direction>)>,
        ),
    >,
) {
    for (fragment_type, direction, mut sprite) in fragments_query.iter_mut() {
        sprite.index = animation_index(fragment_type, direction);
    }
}

pub fn init_snake_sprite_sheet(
    image_assets: Res<AssetServer>,
    mut sprite_sheet: ResMut<SnakeSpriteSheet>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let texture_handle = image_assets.load("snake_pink.png");
    let texture_atlas =
        TextureAtlas::from_grid(texture_handle, Vec2::new(64.0, 64.0), 5, 4, None, None);
    let texture_atlas_handle = texture_atlases.add(texture_atlas);
    sprite_sheet.0 = SpriteSheetBundle {
        sprite: TextureAtlasSprite {
            index: 4,
            custom_size: Some(Vec2::new(1.0, 1.0)),
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, -1.0),
        texture_atlas: texture_atlas_handle,
        ..default()
    };
}
