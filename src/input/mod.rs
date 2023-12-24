mod components;
mod resources;
mod systems;

use bevy::prelude::*;

pub use components::MovementDirection;
pub use resources::TurnRequestsBuffer;
pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, systems::handle_input)
            .insert_resource(TurnRequestsBuffer::new());
    }
}
