use bevy::prelude::*;

#[derive(Debug, Hash, PartialEq, Eq, Clone, SystemSet)]
pub enum GameSystemSets {
    FieldSetup,
    PlayerSetup,
}
