use bevy::prelude::*;

#[derive(Component, Copy, Clone, Debug)]
pub enum RequestDirection {
    Left,
    Right,
    Up,
    Down,
}
