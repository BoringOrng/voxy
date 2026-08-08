use bevy::prelude::*;
use vx_world::chunk::ChunkPos;

mod plugin;

pub use plugin::EntityPlugin;

#[derive(Clone, Copy, Default, PartialEq, Eq, Component)]
#[require(Transform, ChunkPos)]
pub struct VxEntity;
