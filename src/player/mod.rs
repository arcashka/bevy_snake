mod components;
mod helpers;
mod systems;

use bevy::prelude::*;

use systems::{handle_input, move_body, move_head, setup};

use crate::system_sets::GameSystemSets;

pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup.in_set(GameSystemSets::PlayerSetup))
            .add_systems(FixedUpdate, handle_input)
            .add_systems(Update, (move_head, move_body).chain());
    }
}
