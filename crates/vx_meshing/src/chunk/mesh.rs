use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use crate::{Quad, block::BlockMaterial};

pub struct ChunkMesh {
    inner: Mesh,
}

impl ChunkMesh {
    /// # Panics
    ///
    /// Panics if the number of quads * 4 overflows an `u32`.
    pub fn from_quads<I>(quads: I) -> Self
    where
        I: IntoIterator<Item = (Quad, u32)>,
    {
        let mut positions = Vec::new();
        let mut mesh_normals = Vec::new();
        let mut uvs = Vec::new();
        let mut layers = Vec::new();
        let mut indices = Vec::new();

        for (i, (quad, layer)) in quads.into_iter().enumerate() {
            positions.extend_from_slice(&quad.positions());
            mesh_normals.extend_from_slice(&quad.normals());
            uvs.extend_from_slice(&quad.uvs());
            layers.extend_from_slice(&[layer; 4]);

            let base = u32::try_from(i * 4)
                .expect("mesh shouldn't contain enough quads to overflow an `u32`");

            indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
        }

        let mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_normals)
        .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uvs)
        .with_inserted_attribute(BlockMaterial::ATTRIBUTE_LAYER, layers)
        .with_inserted_indices(Indices::U32(indices));

        Self { inner: mesh }
    }
}

impl From<ChunkMesh> for Mesh {
    fn from(value: ChunkMesh) -> Self {
        value.inner
    }
}
