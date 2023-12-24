use bevy::prelude::*;

use super::MovementDirection;
use super::TurnRequestsBuffer;

pub fn handle_input(
    key: Res<Input<KeyCode>>,
    mut direction_change_requests: ResMut<TurnRequestsBuffer>,
) {
    let new_direction = if key.just_pressed(KeyCode::Left) {
        Some(MovementDirection::Left)
    } else if key.just_pressed(KeyCode::Right) {
        Some(MovementDirection::Right)
    } else if key.just_pressed(KeyCode::Up) {
        Some(MovementDirection::Up)
    } else if key.just_pressed(KeyCode::Down) {
        Some(MovementDirection::Down)
    } else {
        None
    };

    if let Some(direction) = new_direction {
        direction_change_requests.push(direction);
    }
}
