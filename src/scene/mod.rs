mod systems;

use bevy::prelude::Event;
use bevy::prelude::Vec2;

#[derive(Event)]
pub struct SceneResizeEvent {
    pub size: Vec2,
}

pub use systems::setup;
pub use systems::window_events_listener;
