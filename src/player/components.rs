use std::f32::consts::PI;

use bevy::prelude::*;

use super::helpers::Direction;
use crate::input::RequestDirection;

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

impl RequestDirection {
    pub fn radians(&self) -> f32 {
        match self {
            RequestDirection::Left => PI,
            RequestDirection::Right => 0.0,
            RequestDirection::Up => PI / 2.0,
            RequestDirection::Down => 3.0 * PI / 2.0,
        }
    }
}

#[derive(Component)]
pub struct Turning(pub Option<TurningValue>);
