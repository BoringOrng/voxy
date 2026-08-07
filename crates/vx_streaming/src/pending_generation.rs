use bevy::prelude::*;
use vx_world::chunk::Chunk;

#[derive(Component)]
#[require(Chunk)]
pub struct PendingGeneration;
