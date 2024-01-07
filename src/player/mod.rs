mod components;
mod events;
mod helpers;
mod resources;
mod systems;

use bevy::prelude::*;

use systems::{check_if_on_new_cell, handle_input, move_body, move_head, setup};

use crate::field::Cell;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(FixedUpdate, handle_input)
            .add_systems(
                Update,
                (
                    move_head,
                    move_body.after(move_head),
                    check_if_on_new_cell,
                    handle_input.after(check_if_on_new_cell),
                ),
            )
            .add_event::<events::MovedOntoNewCellEvent>()
            .insert_resource(resources::PlayerStartSetting {
                cell: Cell::new(4, 4),
                direction: components::Direction::Right,
                speed: 3.0,
                gap: 0.5,
            });
    }
}
