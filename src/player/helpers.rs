use std::f32::consts::PI;

use bevy::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
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
