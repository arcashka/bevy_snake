use super::components::Direction;
use crate::field::Cell;

use bevy::prelude::*;

#[derive(Resource)]
pub struct PlayerStartSetting {
    pub cell: Cell,
    pub direction: Direction,
    pub speed: f32,
    pub gap: f32,
}
