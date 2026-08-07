use bevy::prelude::*;
use vx_world::chunk::ChunkPos;

#[derive(Component, Debug, Default, Clone, Copy)]
#[require(ChunkPos)]
pub struct Anchor;
