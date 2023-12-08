use bevy::prelude::*;

use super::Direction;
use super::FragmentType;

fn animation_index(fragment_type: FragmentType, direction: Direction) -> usize {
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

fn update_snake_sprite(fragments_query: Query<Entity>) {}

pub struct SnakeSpritePlugin;
impl Plugin for SnakeSpritePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, update_snake_sprite);
    }
}
