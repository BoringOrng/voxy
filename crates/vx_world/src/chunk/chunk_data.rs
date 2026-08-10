use bevy::prelude::*;
use vx_datastructure::PVec;

use crate::block::{BlockId, BlockPos};

#[derive(Debug, Default, Clone, Component)]
pub struct ChunkData {
    blocks: PVec<BlockId>,
}

impl ChunkData {
    pub fn iter(&self) -> impl Iterator<Item = (BlockPos, BlockId)> {
        self.blocks
            .iter()
            .map(|(idx, block_id)| (BlockPos::from_raw(idx), block_id))
    }

    #[must_use]
    pub fn block_at(&self, pos: BlockPos) -> bool {
        self.blocks.get(pos.raw()).is_some()
    }

    pub fn insert(&mut self, pos: BlockPos, block: BlockId) -> Option<BlockId> {
        self.blocks.set(pos.raw(), block)
    }

    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.blocks.is_empty()
    }
}
