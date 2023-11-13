use bevy::prelude::*;

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    pos: IVec2,
}

impl Cell {
    pub fn new(x: i32, y: i32) -> Self {
        Self {
            pos: IVec2::new(x, y),
        }
    }

    pub fn i(&self) -> i32 {
        self.pos.x
    }

    pub fn j(&self) -> i32 {
        self.pos.y
    }
}
