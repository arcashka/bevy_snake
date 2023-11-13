use crate::common_types::Cell;
use crate::field_plugin::Field;

use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
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
        Cell::new(wrap(i, self.dimensions().x), wrap(j, self.dimensions().y))
    }

    pub fn single_step_into(&self, from: &Cell, direction: &Direction) -> Cell {
        self.step_into(from, direction, 1)
    }

    pub fn translation_of_position(&self, position: &Cell) -> Vec2 {
        let bottom_left = self.bottom_left();
        let cell_size = self.cell_size();

        let x = bottom_left.x + (position.i() as f32 * cell_size) + (cell_size / 2.0);
        let y = bottom_left.y + (position.j() as f32 * cell_size) + (cell_size / 2.0);
        Vec2::new(x, y)
    }

    fn bottom_left(&self) -> Vec2 {
        let (h, w) = self.size().into();
        let bootom_left_x = -w / 2.0;
        let bootom_left_y = -h / 2.0;
        Vec2::new(bootom_left_x, bootom_left_y)
    }
}
