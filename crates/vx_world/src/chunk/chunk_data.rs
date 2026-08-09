use bevy::prelude::*;

use crate::{
    block::{BlockId, BlockPos},
    chunk::Chunk,
};

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

    #[must_use]
    pub const fn block_at(&self, pos: BlockPos) -> bool {
        self.blocks()[pos.raw() as usize].is_some()
    }

    pub const fn insert(&mut self, pos: BlockPos, block: BlockId) -> Option<BlockId> {
        self.blocks[pos.raw() as usize].replace(block)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.blocks.iter().all(Option::is_none)
    }
}

impl Default for ChunkData {
    fn default() -> Self {
        Self {
            blocks: vec![None; Chunk::VOLUME].into_boxed_slice(),
        }
    }
}
