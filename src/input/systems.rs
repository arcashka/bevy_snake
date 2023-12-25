use bevy::prelude::*;

use super::RequestDirection;
use super::TurnRequestsBuffer;

pub fn handle_input(
    key: Res<Input<KeyCode>>,
    mut direction_change_requests: ResMut<TurnRequestsBuffer>,
) {
    let new_direction = if key.just_pressed(KeyCode::Left) {
        Some(RequestDirection::Left)
    } else if key.just_pressed(KeyCode::Right) {
        Some(RequestDirection::Right)
    } else if key.just_pressed(KeyCode::Up) {
        Some(RequestDirection::Up)
    } else if key.just_pressed(KeyCode::Down) {
        Some(RequestDirection::Down)
    } else {
        None
    };

    if let Some(direction) = new_direction {
        direction_change_requests.push(direction);
    }
}
