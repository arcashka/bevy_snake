use std::collections::VecDeque;

use super::Direction;
use bevy::prelude::*;

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
        info!("new input direction: {:?}", direction);
        direction_change_requests.push(direction);
    }
}

#[derive(Resource)]
pub struct TurnRequestsBuffer {
    buffer: VecDeque<super::Direction>,
}

impl TurnRequestsBuffer {
    pub fn pop(&mut self) -> Option<super::Direction> {
        self.buffer.pop_front()
    }

    pub fn push(&mut self, direction: super::Direction) {
        self.buffer.push_back(direction)
    }

    fn new() -> Self {
        Self {
            buffer: VecDeque::new(),
        }
    }
}

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_input)
            .insert_resource(TurnRequestsBuffer::new());
    }
}
