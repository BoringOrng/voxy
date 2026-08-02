use std::num::NonZeroU16;

use bevy::{platform::collections::HashMap, prelude::*};
use vx_world::block::BlockId;

use crate::block::Block;

#[derive(Resource, Default)]
pub struct BlockRegistry {
    by_id: HashMap<String, BlockId>,
    by_block_id: Vec<Block>,
}

impl BlockRegistry {
    pub fn build(blocks: HashMap<String, Block>) -> Self {
        let mut sorted: Vec<_> = blocks.into_iter().collect();
        sorted.sort_by(|(a, _), (b, _)| a.cmp(b));

        let mut by_id = HashMap::new();
        let mut by_block_id = Vec::new();

        for (i, (string_id, block)) in sorted.into_iter().enumerate() {
            let Ok(raw) = u16::try_from(i + 1) else {
                warn!(
                    "Too many blocks; truncating to the nearest {} blocks",
                    u16::MAX - 1
                );
                break;
            };

            // SAFETY: raw is always incremented so the smallest value is never zero.
            let block_id = BlockId::from_raw(unsafe { NonZeroU16::new_unchecked(raw) });

            by_id.insert(string_id, block_id);
            by_block_id.push(block);
        }

        Self { by_id, by_block_id }
    }

    #[must_use]
    pub fn get_id(&self, name: &str) -> Option<BlockId> {
        self.by_id.get(name).copied()
    }

    #[must_use]
    pub fn get_block(&self, id: BlockId) -> &Block {
        &self.by_block_id[id.raw().get() as usize - 1]
    }

    #[must_use]
    pub fn get_block_by_name(&self, name: &str) -> Option<&Block> {
        self.get_id(name).map(|id| self.get_block(id))
    }

    #[must_use]
    pub const fn len(&self) -> usize {
        self.by_block_id.len()
    }

    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.by_block_id.is_empty()
    }
}
