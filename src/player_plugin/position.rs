use crate::field_plugin::{Cell, Field};

use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn opposite(&self) -> Self {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

impl Field {
    pub fn step_into(&self, from: &Cell, direction: &Direction, step: i32) -> Cell {
        let mut i = from.i();
        let mut j = from.j();
        match direction {
            Direction::Left => i -= step,
            Direction::Right => i += step,
            Direction::Up => j += step,
            Direction::Down => j -= step,
        };
        fn wrap(x: i32, max: i32) -> i32 {
            if x < 0 {
                max + x
            } else if x >= max {
                x - max
            } else {
                x
            }
        }
        Cell::new(wrap(i, self.dimensions.x), wrap(j, self.dimensions.y))
    }

    pub fn single_step_into(&self, from: &Cell, direction: &Direction) -> Cell {
        self.step_into(from, direction, 1)
    }
}
