use bevy::prelude::*;
use vx_mod_behavior::block::{Block, BlockGeometry, BlockRegistry, geometry::SidesTexture};
use vx_world::{
    block::BlockPos,
    chunk::{ChunkData, ChunkMap, ChunkPos},
};

use crate::{
    Quad,
    block::{self, BlockSampler, Face, SharedBlockMaterial},
    chunk::{DirtyChunk, mesh::ChunkMesh},
};

#[derive(Clone, Copy, Default)]
pub struct ChunkMeshingPlugin;

#[derive(Component)]
struct PendingMesh(Vec<Quad>);

impl ChunkMeshingPlugin {
    #[expect(
        clippy::needless_pass_by_value,
        clippy::cast_possible_truncation,
        reason = "
            -  `Res<ChunkMap>` and `Res<BlockRegistry>` must be passed by value as
                is required by bevy

            -  `i as u16` is valid because `32^3` is the max number of blocks in
                a chunk
        "
    )]
    fn generate_quads(
        mut commands: Commands,
        block_registry: If<Res<BlockRegistry>>,
        block_texture_array: If<Res<block::TextureArray>>,
        chunk_map: Res<ChunkMap>,
        chunk_data_q: Query<&ChunkData>,
        dirty_chunks: Query<(Entity, &ChunkPos), With<DirtyChunk>>,
    ) {
        let block_texture_array = &block_texture_array.into_inner();

        for (chunk_entity, &chunk_pos) in &dirty_chunks {
            let chunk_data = chunk_data_q
                .get(chunk_entity)
                .expect("All chunk entities should have an associated `ChunkData`");

            let sampler = &BlockSampler::new(
                chunk_data,
                chunk_map
                    .adjacent_to(chunk_pos)
                    .map(|e| e.and_then(|e| chunk_data_q.get(e).ok())),
            );

            let quads = chunk_data
                .blocks()
                .iter()
                .enumerate()
                .filter_map(|(i, id)| id.map(|id| (BlockPos::from_raw(i as u16), id)))
                .flat_map(|(block_pos, block_id)| {
                    let block = block_registry.get_block(block_id);

                    Face::ALL
                        .into_iter()
                        .filter(move |&face| !sampler.occluded(block_pos, face))
                        .map(move |face| {
                            let texture_id = Self::texture_for_face(block, face);
                            let layer = block_texture_array
                                .get_index(texture_id)
                                .unwrap_or_default();

                            Quad::new(block_pos, layer, face)
                        })
                })
                .collect();

            commands
                .entity(chunk_entity)
                .insert(PendingMesh(quads))
                .remove::<DirtyChunk>();
        }
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "`Res<SharedBlockMaterial>` must be passed by value as is required by bevy"
    )]
    fn build_meshes(
        mut commands: Commands,
        mut meshes: ResMut<Assets<Mesh>>,
        material: If<Res<SharedBlockMaterial>>,
        pending: Query<(Entity, &PendingMesh)>,
    ) {
        for (entity, PendingMesh(quads)) in &pending {
            commands
                .entity(entity)
                .insert(Mesh3d(
                    meshes.add(ChunkMesh::from_quads(quads.iter().copied())),
                ))
                .insert(MeshMaterial3d(material.clone()))
                .remove::<PendingMesh>();
        }
    }

    fn texture_for_face(block: &Block, face: Face) -> &str {
        match (face, block.geometry()) {
            (Face::Up, BlockGeometry::Cube { top, .. }) => top.texture_id(),
            (Face::Down, BlockGeometry::Cube { bottom, .. }) => bottom.texture_id(),
            (
                _,
                BlockGeometry::Cube {
                    sides: SidesTexture::Uniform(sides),
                    ..
                },
            ) => sides.texture_id(),
        }
    }
}

impl Plugin for ChunkMeshingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (Self::generate_quads, Self::build_meshes));
    }
}
