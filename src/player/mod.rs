mod components;
mod events;
mod resources;
mod systems;

use bevy::prelude::*;

use systems::{check_if_on_new_cell, handle_input, move_body, move_head, setup};

use crate::field::Cell;
use crate::states::GameState;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::InGame), setup)
            .add_systems(
                FixedUpdate,
                (
                    move_head,
                    move_body.after(move_head),
                    check_if_on_new_cell,
                    handle_input.after(check_if_on_new_cell),
                )
                    .run_if(in_state(GameState::InGame)),
            )
            .add_event::<events::MovedOntoNewCellEvent>()
            .insert_resource(resources::PlayerStartSetting {
                cell: Cell::new(4, 4),
                direction: components::Direction::Right,
                speed: 3.0,
                gap: 0.1,
            });
    }
}
