use bevy::prelude::*;

use super::Cell;

#[derive(Resource)]
pub struct Field {
    dim: IVec2,
    size: Vec2,
    translation: Vec2,
    cell_size: f32,
}

impl Field {
    pub fn new(dim: IVec2, size: Vec2, translation: Vec2) -> Self {
        let cell_size = size / dim.as_vec2();
        if cell_size.x != cell_size.y {
            panic!("Field must have square cells");
        }
        Self {
            dim,
            size,
            translation,
            cell_size: cell_size.x,
        }
    }

    #[allow(dead_code)]
    pub fn translation_of_cell(&self, position: &Cell) -> Vec2 {
        let bottom_left = self.bottom_left();
        let cell_size = self.cell_size();

        let x = bottom_left.x + (position.i() as f32 * cell_size) + (cell_size / 2.0);
        let y = bottom_left.y + (position.j() as f32 * cell_size) + (cell_size / 2.0);
        Vec2 { x, y }
    }

    pub fn cell(&self, translation: Vec2) -> Cell {
        let cell_size = self.cell_size();
        let local_translation = translation - self.bottom_left();
        Cell {
            pos: IVec2 {
                x: (local_translation.x / cell_size) as i32,
                y: (local_translation.y / cell_size) as i32,
            },
        }
    }

    #[allow(dead_code)]
    pub fn cell_local_translation(&self, translation: Vec2) -> Vec2 {
        (translation - self.translation) % self.cell_size()
    }

    pub fn cell_size(&self) -> f32 {
        self.cell_size
    }

    pub fn dim(&self) -> IVec2 {
        self.dim
    }

    pub fn size(&self) -> Vec2 {
        self.size
    }

    pub fn offset(&self) -> Vec2 {
        self.translation
    }

    fn bottom_left(&self) -> Vec2 {
        let (h, w) = self.size.into();
        Vec2::new(-w / 2.0, -h / 2.0) - self.translation
    }
}

#[test]
fn test_field_translation() {
    use super::*;
    let field = Field::new(IVec2::new(10, 10), Vec2::new(100.0, 100.0), Vec2::ZERO);
    let translation = Vec2::new(27.0, 23.0);
    let cell = field.cell(translation);
    assert_eq!(cell, Cell::new(7, 7));
    assert_eq!(field.translation_of_cell(&cell), Vec2 { x: 25.0, y: 25.0 });
}

#[test]
fn test_field_local_coordinates() {
    use super::*;
    let field = Field::new(IVec2::new(10, 10), Vec2::new(100.0, 100.0), Vec2::ZERO);
    let translation = Vec2::new(27.0, 23.0);
    let local_translation = field.cell_local_translation(translation);
    assert_eq!(local_translation, Vec2 { x: 7.0, y: 3.0 });
}
