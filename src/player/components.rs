use bevy::prelude::*;

use super::helpers::Direction;
use crate::input::RequestDirection;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct TurnSpeed(pub f32);

#[derive(Component)]
pub struct Fragment(pub u32);

#[derive(Component)]
pub struct DistanceFromStart(pub f32);

#[derive(Component)]
pub struct BodyInfo {
    pub body: Vec<Entity>,
    pub first_gap: f32,
    pub gap: f32,
}

#[derive(Component)]
pub struct PreviousTransforms {
    pub body: Vec<Transform>,
}

pub enum TurnDirection {
    Left,
    Right,
}

impl TurnDirection {
    pub fn from_turn_request(
        current: Direction,
        request: RequestDirection,
    ) -> Option<TurnDirection> {
        match current {
            Direction::Left => match request {
                RequestDirection::Up => Some(TurnDirection::Left),
                RequestDirection::Down => Some(TurnDirection::Right),
                RequestDirection::Right => Some(TurnDirection::Right),
                _ => None,
            },
            Direction::Right => match request {
                RequestDirection::Up => Some(TurnDirection::Right),
                RequestDirection::Down => Some(TurnDirection::Left),
                RequestDirection::Left => Some(TurnDirection::Left),
                _ => None,
            },
            Direction::Up => match request {
                RequestDirection::Left => Some(TurnDirection::Right),
                RequestDirection::Right => Some(TurnDirection::Left),
                RequestDirection::Down => Some(TurnDirection::Right),
                _ => None,
            },
            Direction::Down => match request {
                RequestDirection::Left => Some(TurnDirection::Left),
                RequestDirection::Right => Some(TurnDirection::Right),
                RequestDirection::Up => Some(TurnDirection::Left),
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
    pub progress: f32,
}

#[derive(Component)]
pub struct Turning(pub Option<TurningValue>);
