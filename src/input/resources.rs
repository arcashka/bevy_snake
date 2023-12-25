use bevy::prelude::*;

use super::RequestDirection;
use std::collections::VecDeque;

#[derive(Resource)]
pub struct TurnRequestsBuffer {
    buffer: VecDeque<RequestDirection>,
}

impl TurnRequestsBuffer {
    pub fn pop(&mut self) -> Option<RequestDirection> {
        self.buffer.pop_front()
    }

    pub fn push(&mut self, direction: RequestDirection) {
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
