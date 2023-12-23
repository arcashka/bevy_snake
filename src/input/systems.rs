use bevy::prelude::*;

use super::resources::TurnRequestsBuffer;
use crate::player::Direction;

pub fn handle_input(
    key: Res<Input<KeyCode>>,
    mut direction_change_requests: ResMut<TurnRequestsBuffer>,
) {
    let new_direction = if key.just_pressed(KeyCode::Left) {
        Some(Direction::Left)
    } else if key.just_pressed(KeyCode::Right) {
        Some(Direction::Right)
    } else if key.just_pressed(KeyCode::Up) {
        Some(Direction::Up)
    } else if key.just_pressed(KeyCode::Down) {
        Some(Direction::Down)
    } else {
        None
    };

    if let Some(direction) = new_direction {
        direction_change_requests.push(direction);
    }
}
