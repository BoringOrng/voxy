use std::collections::BinaryHeap;

use bevy::{platform::collections::HashSet, prelude::*};
use vx_world::chunk::{ChunkMap, ChunkPos};

mod item;

pub use item::Item;

#[derive(Resource, Default)]
pub struct Queue {
    heap: BinaryHeap<Item>,
    queued: HashSet<ChunkPos>,
}

impl Queue {
    pub fn push(&mut self, item: Item) -> bool {
        if self.queued.insert(item.pos()) {
            self.heap.push(item);

            true
        } else {
            false
        }
    }

    pub fn remove(&mut self, pos: &ChunkPos) -> bool {
        self.queued.remove(pos)
    }

    pub fn retain<F>(&mut self, f: F)
    where
        F: FnMut(&ChunkPos) -> bool,
    {
        self.queued.retain(f);
    }

    pub fn pop_batch(&mut self, n: usize, chunk_map: &ChunkMap) -> Vec<ChunkPos> {
        let mut out = Vec::with_capacity(n);

        while out.len() < n {
            let Some(item) = self.heap.pop() else {
                break;
            };

            let pos = item.pos();

            if !self.queued.remove(&pos) || chunk_map.contains_key(&pos) {
                continue;
            }

            out.push(pos);
        }

        out
    }
}
