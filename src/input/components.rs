use bevy::prelude::*;

#[derive(Component, Copy, Clone, Debug)]
pub enum MovementDirection {
    Left,
    Right,
    Up,
    Down,
}
