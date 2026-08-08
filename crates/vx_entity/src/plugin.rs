use bevy::prelude::*;
use vx_world::chunk::ChunkPos;

pub struct EntityPlugin;

impl EntityPlugin {
    fn sync_chunk_pos(mut entities: Query<(&Transform, &mut ChunkPos), With<super::VxEntity>>) {
        for (transform, mut chunk_pos) in &mut entities {
            *chunk_pos = ChunkPos::from_vec3(transform.translation);
        }
    }
}

impl Plugin for EntityPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreUpdate, Self::sync_chunk_pos);
    }
}
