use bevy::prelude::*;

use crate::chunk::{ChunkMap, ChunkPos};

#[derive(Default)]
pub struct ChunkPlugin;

impl ChunkPlugin {
    fn map_translation_to_chunk_pos(mut positions: Query<(&Transform, &mut ChunkPos)>) {
        for (transform, mut old_pos) in &mut positions {
            let cur_pos = ChunkPos::from_vec3(transform.translation);
            *old_pos = cur_pos;
        }
    }
}

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkMap>()
            .add_systems(PreUpdate, Self::map_translation_to_chunk_pos);
    }
}
