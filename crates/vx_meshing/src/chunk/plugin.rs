use bevy::prelude::*;
use vx_world::{
    block::BlockPos,
    chunk::{ChunkData, ChunkMap, ChunkPos},
};

use crate::{
    Quad,
    block::{BlockSampler, Face},
    chunk::{DirtyChunk, mesh::ChunkMesh},
};

#[derive(Clone, Copy, Default)]
pub struct ChunkMeshingPlugin;

impl ChunkMeshingPlugin {
    #[expect(
        clippy::needless_pass_by_value,
        reason = "`Res<ChunkMap>` must be passed by value as is required by bevy"
    )]
    fn mesh_dirty(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        chunk_map: Res<ChunkMap>,
        chunk_data_q: Query<&ChunkData>,
        dirty_chunks: Query<(Entity, &ChunkPos), With<DirtyChunk>>,
    ) {
        for (chunk_entity, &chunk_pos) in &dirty_chunks {
            let chunk_data = chunk_data_q
                .get(chunk_entity)
                .expect("All chunk entities should have an associated `ChunkData`");

            let sampler = BlockSampler::new(
                chunk_data,
                chunk_map
                    .adjacent_to(chunk_pos)
                    .map(|e| e.and_then(|e| chunk_data_q.get(e).ok())),
            );

            let blocks = chunk_data.blocks();
            let mut quads = Vec::new();

            #[expect(
                clippy::cast_possible_truncation,
                reason = "an index into the block data can never exceed `u16` precision"
            )]
            for (block_pos, block_id) in blocks
                .iter()
                .enumerate()
                .filter_map(|(i, id)| id.map(|id| (BlockPos::from_raw(i as u16), id)))
            {
                // TODO: texture sampling
                _ = block_id;

                Face::ALL
                    .iter()
                    .filter(|&&face| !sampler.occluded(block_pos, face))
                    .map(|&face| Quad::new(block_pos, face))
                    .collect_into(&mut quads);
            }

            commands
                .entity(chunk_entity)
                .insert(Mesh3d(meshes.add(ChunkMesh::from_quads(quads))))
                .insert(MeshMaterial3d(materials.add(Color::srgb(1.0, 0.0, 1.0))))
                .remove::<DirtyChunk>();
        }
    }
}

impl Plugin for ChunkMeshingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::mesh_dirty);
    }
}
