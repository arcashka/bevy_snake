use bevy::prelude::*;

#[derive(Component)]
pub struct Field {
    pub dimensions: IVec2,
    pub size: Vec2,
    pub offset: Vec2,
}

impl Field {
    #[allow(dead_code)]
    pub fn translation(&self, position: &Cell) -> Vec2 {
        let bottom_left = self.bottom_left();
        let cell_size = self.cell_size();

        let x = bottom_left.x + (position.i() as f32 * cell_size.x) + (cell_size.x / 2.0);
        let y = bottom_left.y + (position.j() as f32 * cell_size.y) + (cell_size.y / 2.0);
        Vec2 { x, y }
    }

    pub fn cell(&self, translation: Vec2) -> Cell {
        let cell_size = self.cell_size();
        let local_translation = translation - self.bottom_left();
        Cell {
            pos: IVec2 {
                x: (local_translation.x / cell_size.x) as i32,
                y: (local_translation.y / cell_size.y) as i32,
            },
        }
    }

    fn bottom_left(&self) -> Vec2 {
        let (h, w) = self.size.into();
        Vec2::new(-w / 2.0, -h / 2.0)
    }

    fn cell_size(&self) -> Vec2 {
        self.size / self.dimensions.as_vec2()
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

#[test]
fn test_field() {
    use super::*;
    let field = Field {
        dimensions: IVec2::new(10, 10),
        size: Vec2::new(100.0, 100.0),
        offset: Vec2::ZERO,
    };
    let translation = Vec2::new(27.0, 23.0);
    let cell = field.cell(translation);
    assert_eq!(cell, Cell::new(7, 7));
    assert_eq!(field.translation(&cell), Vec2 { x: 25.0, y: 25.0 });
}
