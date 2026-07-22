use bevy::prelude::*;

use crate::{block::BlockId, chunk::Chunk};

#[derive(Debug, Clone, Component)]
pub struct ChunkData {
    blocks: Box<[Option<BlockId>]>,
}

impl ChunkData {
    #[must_use]
    pub const fn blocks(&self) -> &[Option<BlockId>; Chunk::VOLUME] {
        // SAFETY: constructors should derive from the default constructor, which
        // guarantees an allocated size of `Chunk::VOLUME`
        unsafe { self.blocks.as_array().unwrap_unchecked() }
    }
}

impl Default for ChunkData {
    fn default() -> Self {
        Self {
            blocks: vec![None; Chunk::VOLUME].into_boxed_slice(),
        }
    }
}
