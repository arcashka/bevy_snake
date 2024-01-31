use bevy::prelude::*;

use super::RequestDirection;
use super::TurnRequestsBuffer;

pub fn handle_input(
    key: Res<ButtonInput<KeyCode>>,
    mut direction_change_requests: ResMut<TurnRequestsBuffer>,
) {
    let new_direction = if key.just_pressed(KeyCode::ArrowLeft) {
        Some(RequestDirection::Left)
    } else if key.just_pressed(KeyCode::ArrowRight) {
        Some(RequestDirection::Right)
    } else if key.just_pressed(KeyCode::ArrowUp) {
        Some(RequestDirection::Up)
    } else if key.just_pressed(KeyCode::ArrowDown) {
        Some(RequestDirection::Down)
    } else {
        None
    };

    if let Some(direction) = new_direction {
        direction_change_requests.push(direction);
    }
}
