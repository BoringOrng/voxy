use bevy::prelude::*;
use vx_world::chunk::ChunkPos;

#[derive(Component, Default)]
pub struct LastPos(pub(crate) Option<IVec3>);

#[derive(Component, Debug, Default, Clone, Copy)]
#[require(ChunkPos, LastPos)]
pub struct Anchor;
