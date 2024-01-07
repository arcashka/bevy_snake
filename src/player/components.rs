use bevy::prelude::*;

use std::f32::consts::PI;

use crate::input::RequestDirection;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Speed(pub f32);

#[derive(Component)]
pub struct TurnSpeed(pub f32);

#[derive(Component)]
pub struct Fragment(pub u32);

#[derive(Component, Copy, Clone, PartialEq, Debug)]
pub struct DistancePassed(pub f32);

#[derive(Component)]
pub struct BodyInfo {
    pub body: Vec<Entity>,
    pub first_gap: f32,
    pub gap: f32,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct PreviousHeadPosition {
    pub transform: Transform,
    pub distance_passed: DistancePassed,
}

#[derive(Component)]
pub struct PreviousHeadPositions(pub Vec<PreviousHeadPosition>);

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

#[derive(Component, Clone, Copy, Debug, PartialEq)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn closest_from_rotation(rotation: &Quat) -> Self {
        let rotation_direction = rotation.mul_vec3(Vec3::X);
        let dot = Vec3::X.dot(rotation_direction).clamp(-1.0, 1.0);
        let diff_zero = (dot - 0.0).abs();
        let diff_one = (dot - 1.0).abs();
        let diff_minus_one = (dot + 1.0).abs();

        if diff_one < diff_zero && diff_one < diff_minus_one {
            Direction::Right
        } else if diff_minus_one < diff_zero {
            Direction::Left
        } else {
            // dot is closest to 0.0, distinguish between Up and Down
            if rotation_direction.z > 0.0 {
                Direction::Down
            } else {
                Direction::Up
            }
        }
    }

    pub fn quaternion(&self) -> Quat {
        let angle = match self {
            Direction::Left => PI,
            Direction::Right => 0.0,
            Direction::Up => PI / 2.0,
            Direction::Down => 3.0 * PI / 2.0,
        };
        Quat::from_rotation_y(angle)
    }
}

#[cfg(test)]
mod tests {
    use super::Direction;
    use bevy::prelude::*;
    use std::f32::consts::PI;

    #[test]
    fn test_closest_from_rotation() {
        let test_cases = vec![
            (Quat::from_rotation_y(0.0), Direction::Right),
            (Quat::from_rotation_y(PI / 2.0), Direction::Up),
            (Quat::from_rotation_y(PI), Direction::Left),
            (Quat::from_rotation_y(3.0 * PI / 2.0), Direction::Down),
            // Test with negative angle
            (Quat::from_rotation_y(-PI / 2.0), Direction::Down),
            (Quat::from_rotation_y(5.0 * PI / 2.0), Direction::Up),
            (Quat::from_rotation_y(7.0 * PI / 2.0), Direction::Down),
            (Quat::from_rotation_y(-3.0 * PI / 2.0), Direction::Up),
            (Quat::from_rotation_y(-7.0 * PI / 2.0), Direction::Up),
        ];

        for (quat, expected_direction) in test_cases {
            assert_eq!(Direction::closest_from_rotation(&quat), expected_direction);
        }
    }

    #[test]
    fn test_quaternion() {
        let test_cases = vec![
            (Direction::Right, Quat::from_rotation_y(0.0)),
            (Direction::Up, Quat::from_rotation_y(PI / 2.0)),
            (Direction::Left, Quat::from_rotation_y(PI)),
            (Direction::Down, Quat::from_rotation_y(3.0 * PI / 2.0)),
        ];

        for (direction, expected_quat) in test_cases {
            let quat = direction.quaternion();
            assert!(quat.abs_diff_eq(expected_quat, 1e-6));
        }
    }
}
