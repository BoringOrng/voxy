use bevy::prelude::*;
use vx_world::{block::BlockPos, chunk::ChunkData};

use crate::{
    Quad,
    block::{BlockSampler, Face},
    chunk::{DirtyChunk, mesh::ChunkMesh},
};

#[derive(Clone, Copy, Default)]
pub struct ChunkMeshingPlugin;

impl ChunkMeshingPlugin {
    fn mesh_dirty(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        mut materials: ResMut<Assets<StandardMaterial>>,
        dirty_chunks: Query<(Entity, &ChunkData), With<DirtyChunk>>,
    ) {
        for (chunk_entity, chunk_data) in &dirty_chunks {
            // TODO: use actual `ChunkMap`
            let sampler = BlockSampler::new(chunk_data, [None; 6]);
            let blocks = chunk_data.blocks();

            let mut quads = Vec::new();

            #[expect(
                clippy::cast_possible_truncation,
                reason = "an index into the block data can never exceed `u16` precision"
            )]
            let occupancies = blocks
                .iter()
                .enumerate()
                .filter_map(|(i, id)| id.map(|id| (BlockPos::from_raw(i as u16), id)))
                .collect::<Vec<_>>();

            for (block_pos, block_id) in occupancies {
                // TODO: texture sampling
                _ = block_id;

                for &face in Face::ALL
                    .iter()
                    .filter(|&&face| !sampler.occluded(block_pos, face))
                {
                    quads.push(Quad::new(block_pos, face));
                }
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
