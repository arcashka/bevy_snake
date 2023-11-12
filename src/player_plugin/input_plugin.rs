use super::Direction;
use bevy::prelude::*;

#[derive(Event, Debug)]
pub struct ChangeDirectionRequested {
    pub new_direction: super::Direction,
}

pub fn handle_input(
    key: Res<Input<KeyCode>>,
    mut change_direction_events: EventWriter<ChangeDirectionRequested>,
) {
    let new_direction = if key.pressed(KeyCode::Left) {
        Some(Direction::Left)
    } else if key.pressed(KeyCode::Right) {
        Some(Direction::Right)
    } else if key.pressed(KeyCode::Up) {
        Some(Direction::Up)
    } else if key.pressed(KeyCode::Down) {
        Some(Direction::Down)
    } else {
        None
    };

    if let Some(direction) = new_direction {
        change_direction_events.send(ChangeDirectionRequested {
            new_direction: direction,
        });
    }
}

pub struct InputPlugin;
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, handle_input)
            .init_resource::<Events<ChangeDirectionRequested>>();
    }
}
