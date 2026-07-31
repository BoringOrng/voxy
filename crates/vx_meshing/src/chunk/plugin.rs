use bevy::{mesh::MeshTag, prelude::*};
use vx_mod_behavior::block::{Block, BlockGeometry, BlockRegistry, geometry::SidesTexture};
use vx_mod_resource::BlockTextureArray;
use vx_world::{
    block::BlockPos,
    chunk::{ChunkData, ChunkMap, ChunkPos},
};

use crate::{
    Quad,
    block::{BlockMaterial, BlockSampler, Face},
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
        mut materials: ResMut<Assets<BlockMaterial>>,
        block_material: Res<BlockMaterial>,
        block_registry: Res<BlockRegistry>,
        block_texture_array: Res<BlockTextureArray>,
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
                let block = block_registry.get_block(block_id);

                Face::ALL
                    .iter()
                    .filter(|&&face| !sampler.occluded(block_pos, face))
                    .map(|&face| {
                        let texture_id = Self::texture_for_face(block, face);
                        let layer = block_texture_array
                            .get_index(texture_id)
                            // should we do this here or just panic?
                            .unwrap_or_default();

                        (Quad::new(block_pos, face), layer)
                    })
                    .collect_into(&mut quads);
            }

            commands
                .entity(chunk_entity)
                .insert(Mesh3d(meshes.add(ChunkMesh::from_quads(quads))))
                .insert(MeshTag(0))
                .insert(MeshMaterial3d(materials.add(block_material.clone())))
                .remove::<DirtyChunk>();
        }
    }

    fn setup_block_material(block_texture_array: Res<BlockTextureArray>, mut commands: Commands) {
        commands.insert_resource(BlockMaterial {
            texture: block_texture_array.into_inner().texture_handle().clone(),
        });
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
        app.add_plugins(MaterialPlugin::<BlockMaterial>::default())
            .add_systems(
                Update,
                (
                    Self::mesh_dirty.run_if(resource_exists::<BlockMaterial>),
                    Self::setup_block_material.run_if(not(resource_exists::<BlockMaterial>)),
                )
                    .run_if(resource_exists::<BlockTextureArray>),
            );
    }
}
