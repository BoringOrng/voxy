use bevy::{platform::collections::HashMap, prelude::*};

use crate::chunk::ChunkPos;

#[derive(Clone, Debug, Default, Resource)]
pub struct ChunkMap {
    map: HashMap<ChunkPos, Entity>,
}

impl ChunkMap {
    pub fn insert(&mut self, pos: ChunkPos, ent: Entity) {
        self.map.insert(pos, ent);
    }

    pub fn remove(&mut self, pos: &ChunkPos) -> Option<Entity> {
        self.map.remove(pos)
    }
}
