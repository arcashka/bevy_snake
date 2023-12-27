use bevy::prelude::*;

#[derive(Component)]
pub struct Field {
    pub dimensions: IVec2,
}

impl Field {
    #[allow(dead_code)]
    pub fn translation(&self, cell: &Cell) -> Vec2 {
        Vec2 {
            x: cell.i() as f32 - self.dimensions.x as f32 / 2.0 + 0.5,
            y: cell.j() as f32 - self.dimensions.y as f32 / 2.0 + 0.5,
        }
    }
}

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    pub pos: IVec2,
}

#[allow(dead_code)]
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

#[derive(Component, Clone, Copy, Eq, PartialEq, Debug)]
pub struct FieldId(pub i32);
