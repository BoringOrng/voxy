use bevy::{platform::collections::HashMap, prelude::*};

use crate::chunk::ChunkPos;

#[derive(Clone, Debug, Deref, DerefMut, Default, Resource)]
pub struct ChunkMap {
    map: HashMap<ChunkPos, Entity>,
}

impl ChunkMap {
    #[must_use]
    pub fn adjacent_to(&self, pos: ChunkPos) -> [Option<Entity>; 6] {
        [
            self.map.get(&pos.up()).copied(),
            self.map.get(&pos.down()).copied(),
            self.map.get(&pos.north()).copied(),
            self.map.get(&pos.south()).copied(),
            self.map.get(&pos.east()).copied(),
            self.map.get(&pos.west()).copied(),
        ]
    }
}
