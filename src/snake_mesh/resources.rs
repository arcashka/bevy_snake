use bevy::{prelude::*, utils::EntityHashMap};

use super::components::SnakeMeshInstance;

#[derive(Default, Resource, Deref, DerefMut)]
pub struct SnakeMeshInstances(EntityHashMap<Entity, SnakeMeshInstance>);
