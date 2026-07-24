use vx_world::{block::BlockPos, chunk::ChunkData};

use crate::block::Face;

pub struct BlockSampler<'chunks> {
    center: &'chunks ChunkData,
    neighbors: [Option<&'chunks ChunkData>; 6],
}

impl<'chunks> BlockSampler<'chunks> {
    #[must_use]
    pub const fn new(
        center: &'chunks ChunkData,
        neighbors: [Option<&'chunks ChunkData>; 6],
    ) -> Self {
        Self { center, neighbors }
    }

    #[must_use]
    pub fn occluded(&self, pos: BlockPos, face: Face) -> bool {
        match face.offset_pos(pos) {
            Some(check_pos) if self.center.block_at(check_pos) => true,
            None => self
                .neighbor_from_face(face)
                .is_some_and(|n| n.block_at(face.wrap_pos(pos))),
            _ => false,
        }
    }

    #[must_use]
    pub const fn neighbor_from_face(&self, face: Face) -> Option<&ChunkData> {
        self.neighbors[face as usize]
    }
}
