use bevy::prelude::*;

#[derive(Component, Default)]
pub struct Field {
    dimensions: IVec2,
    offset: Vec2,
    cell_size: f32,
}

impl Field {
    pub fn new(dimensions: IVec2, offset: Vec2, window_size: Vec2) -> Self {
        Self {
            dimensions,
            offset,
            cell_size: Self::calculate_cell_size(dimensions, offset, window_size),
        }
    }

    pub fn resize(&mut self, new_window_size: Vec2) {
        self.cell_size = Self::calculate_cell_size(self.dimensions, self.offset, new_window_size);
        info!("cell size changed: {}", self.cell_size);
    }

    pub fn size(&self) -> Vec2 {
        Vec2 {
            x: self.cell_size * self.dimensions.x as f32,
            y: self.cell_size * self.dimensions.y as f32,
        }
    }

    pub fn dimensions(&self) -> IVec2 {
        self.dimensions
    }

    pub fn translation(&self) -> Vec2 {
        self.offset
    }

    pub fn cell_size(&self) -> f32 {
        self.cell_size
    }

    fn calculate_cell_size(dimensions: IVec2, offset: Vec2, window_size: Vec2) -> f32 {
        let cell_width = (window_size.x - offset.x) / dimensions.x as f32;
        let cell_height = (window_size.y - offset.y) / dimensions.y as f32;
        cell_width.min(cell_height)
    }
}
