use bevy::prelude::*;

use super::MovementDirection;
use std::collections::VecDeque;

#[derive(Resource)]
pub struct TurnRequestsBuffer {
    buffer: VecDeque<MovementDirection>,
}

impl TurnRequestsBuffer {
    pub fn pop(&mut self) -> Option<MovementDirection> {
        self.buffer.pop_front()
    }

    pub fn push(&mut self, direction: MovementDirection) {
        self.buffer.push_back(direction);
        while self.buffer.len() > 2 {
            self.buffer.pop_front();
        }
    }

    pub fn new() -> Self {
        Self {
            buffer: VecDeque::new(),
        }
    }
}
