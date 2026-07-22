use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, Deref, Component, PartialEq, Eq, Hash)]
pub struct ChunkPos(IVec3);
