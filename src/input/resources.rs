use crate::player::Direction;

use bevy::prelude::*;

use std::collections::VecDeque;

#[derive(Resource)]
pub struct TurnRequestsBuffer {
    buffer: VecDeque<Direction>,
}

impl TurnRequestsBuffer {
    pub fn pop(&mut self) -> Option<Direction> {
        self.buffer.pop_front()
    }

    pub fn push(&mut self, direction: Direction) {
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
