use bevy::prelude::*;
use vx_world::chunk::Chunk;

mod mesh;
mod plugin;

pub use mesh::ChunkMesh;
pub use plugin::ChunkMeshingPlugin;

#[derive(Clone, Copy, Component, Default)]
#[require(Chunk)]
pub struct DirtyChunk;
