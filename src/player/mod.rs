mod components;
mod events;
mod helpers;
mod systems;

use bevy::prelude::*;

use systems::{check_if_on_new_cell, handle_input, move_body, move_head, setup};

use crate::system_sets::GameSystemSets;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup.in_set(GameSystemSets::PlayerSetup))
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
            .add_event::<events::MovedOntoNewCellEvent>();
    }
}
