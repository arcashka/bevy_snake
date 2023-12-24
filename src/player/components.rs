use bevy::prelude::*;

use crate::input::MovementDirection;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct TurnSpeed(pub f32);

pub enum TurnDirection {
    Left,
    Right,
}

impl TurnDirection {
    pub fn from_turn_request(
        current: MovementDirection,
        request: MovementDirection,
    ) -> Option<TurnDirection> {
        match current {
            MovementDirection::Left => match request {
                MovementDirection::Up => Some(TurnDirection::Right),
                MovementDirection::Down => Some(TurnDirection::Left),
                _ => None,
            },
            MovementDirection::Right => match request {
                MovementDirection::Up => Some(TurnDirection::Left),
                MovementDirection::Down => Some(TurnDirection::Right),
                _ => None,
            },
            MovementDirection::Up => match request {
                MovementDirection::Left => Some(TurnDirection::Left),
                MovementDirection::Right => Some(TurnDirection::Right),
                _ => None,
            },
            MovementDirection::Down => match request {
                MovementDirection::Left => Some(TurnDirection::Right),
                MovementDirection::Right => Some(TurnDirection::Left),
                _ => None,
            },
        }
    }

    pub fn sign(&self) -> f32 {
        match self {
            TurnDirection::Left => -1.0,
            TurnDirection::Right => 1.0,
        }
    }
}

pub struct TurningValue {
    pub direction: TurnDirection,
    pub target: MovementDirection,
    pub progress: f32,
}

impl MovementDirection {
    pub fn degree(&self) -> f32 {
        match self {
            MovementDirection::Left => 90.0,
            MovementDirection::Right => 270.0,
            MovementDirection::Up => 0.0,
            MovementDirection::Down => 180.0,
        }
    }
}

#[derive(Component)]
pub struct Turning(pub Option<TurningValue>);
