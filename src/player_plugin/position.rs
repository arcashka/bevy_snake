use crate::common_types::Position;
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
    pub fn step_into(&self, from: &Position, direction: &Direction, step: i32) -> Position {
        let mut i = from.i();
        let mut j = from.j();
        match direction {
            Direction::Left => i -= step,
            Direction::Right => i += step,
            Direction::Up => j -= step,
            Direction::Down => j += step,
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
        Position::new(wrap(i, self.dimensions().x), wrap(j, self.dimensions().y))
    }

    pub fn single_step_into(&self, from: &Position, direction: &Direction) -> Position {
        self.step_into(from, direction, 1)
    }

    pub fn translation_of_position(&self, position: &Position) -> Vec2 {
        let bottom_left = self.bottom_left();
        let cell_size = self.cell_size();

        let x = bottom_left.x + (position.i() as f32 * cell_size) + (cell_size / 2.0);
        let y = bottom_left.y + (position.j() as f32 * cell_size) + (cell_size / 2.0);
        Vec2::new(x, y)
    }

    pub fn position_of_translation(&self, translation: Vec2) -> Position {
        let bottom_left = self.bottom_left();
        let diff = translation - bottom_left;
        Position::new(
            (diff.x / self.cell_size()) as i32,
            (diff.y / self.cell_size()) as i32,
        )
    }

    fn bottom_left(&self) -> Vec2 {
        let (h, w) = self.size().into();
        let bootom_left_x = -w / 2.0;
        let bootom_left_y = -h / 2.0;
        Vec2::new(bootom_left_x, bootom_left_y)
    }
}
