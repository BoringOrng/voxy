use bevy::{
    asset::RenderAssetUsages,
    mesh::{Indices, PrimitiveTopology},
    prelude::*,
};

use crate::Quad;

pub struct ChunkMesh {
    inner: Mesh,
}

impl ChunkMesh {
    pub fn from_quads<I>(quads: I) -> Self
    where
        I: IntoIterator<Item = Quad>,
    {
        let mut positions = Vec::new();
        let mut mesh_normals = Vec::new();
        let mut indices = Vec::new();

        for (i, quad) in quads.into_iter().enumerate() {
            let corners = quad.face().corners().map(|c| c + quad.pos().as_vec3());
            let normals = [quad.face().normal(); 4];

            positions.extend_from_slice(&corners);
            mesh_normals.extend_from_slice(&normals);

            let base = i as u32 * 4;
            indices.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
        }

        let mesh = Mesh::new(
            PrimitiveTopology::TriangleList,
            RenderAssetUsages::RENDER_WORLD,
        )
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, mesh_normals)
        .with_inserted_indices(Indices::U32(indices));

        Self { inner: mesh }
    }
}

impl From<ChunkMesh> for Mesh {
    fn from(value: ChunkMesh) -> Self {
        value.inner
    }
}
