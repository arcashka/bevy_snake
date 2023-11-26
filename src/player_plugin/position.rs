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

    pub fn direction(&self, l: &Cell, r: &Cell) -> Direction {
        let grad = self.grad(r, l);
        if grad.x == 1 && grad.y == 0 {
            return Direction::Right;
        }
        if grad.x == -1 && grad.y == 0 {
            return Direction::Left;
        }
        if grad.x == 0 && grad.y == 1 {
            return Direction::Up;
        }
        if grad.x == 0 && grad.y == -1 {
            return Direction::Down;
        }
        panic!("Unknown direction: {:?}", grad);
    }

    fn grad(&self, l: &Cell, r: &Cell) -> IVec2 {
        let mut dx = l.pos.x - r.pos.x;
        let dimensions = self.dimensions();
        if dx.abs() == dimensions.x - 1 {
            dx /= -(dimensions.x - 1);
        }
        let mut dy = l.pos.y - r.pos.y;
        if dy.abs() == dimensions.y - 1 {
            dy /= -(dimensions.y - 1);
        }
        IVec2 { x: dx, y: dy }
    }

    fn bottom_left(&self) -> Vec2 {
        let (h, w) = self.size().into();
        let bootom_left_x = -w / 2.0;
        let bootom_left_y = -h / 2.0;
        Vec2::new(bootom_left_x, bootom_left_y)
    }
}

#[test]
fn test_direction() {
    let field = Field::new(
        IVec2 { x: 5, y: 5 },
        Vec2::default(),
        Vec2 { x: 100.0, y: 100.0 },
    );
    let cell1 = Cell::new(0, 0);
    let cell2 = Cell::new(1, 0);

    assert_eq!(field.direction(&cell1, &cell2), Direction::Right);
    assert_eq!(field.direction(&cell2, &cell1), Direction::Left);

    let cell3 = Cell::new(0, 3);
    let cell4 = Cell::new(0, 4);

    assert_eq!(field.direction(&cell3, &cell4), Direction::Up);
    assert_eq!(field.direction(&cell4, &cell3), Direction::Down);

    let cell5 = Cell::new(0, 4);
    let cell6 = Cell::new(0, 0);

    assert_eq!(field.direction(&cell5, &cell6), Direction::Up);
    assert_eq!(field.direction(&cell6, &cell5), Direction::Down);
}

#[test]
#[should_panic(expected = "Unknown direction")]
fn test_direction_panic() {
    let field = Field::new(
        IVec2 { x: 5, y: 5 },
        Vec2::default(),
        Vec2 { x: 100.0, y: 100.0 },
    );
    let cell1 = Cell::new(0, 7);
    let cell2 = Cell::new(2, 0); // Same cell, direction unknown

    field.direction(&cell1, &cell2);
}
