use vx_world::{
    block::BlockPos,
    chunk::{Chunk, ChunkData},
};

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
    pub const fn occluded(&self, pos: BlockPos, face: Face) -> bool {
        let to_check = match face {
            Face::Up => pos.up(),
            Face::Down => pos.down(),
            Face::North => pos.north(),
            Face::South => pos.south(),
            Face::East => pos.east(),
            Face::West => pos.west(),
        };

        #[expect(
            clippy::cast_possible_truncation,
            reason = "`Chunk::SIZE::.` shouldn't exceed the precision of a `u8`"
        )]
        match to_check {
            Some(check_pos) if self.center.block_at(check_pos) => true,
            None if let Some(neighboring_chunk) = self.neighbor_from_face(face) => {
                let wrapped_pos = match face {
                    Face::Up => pos.with_y(0),
                    Face::Down => pos.with_y(Chunk::SIZE.y as u8 - 1),
                    Face::North => pos.with_z(0),
                    Face::South => pos.with_z(Chunk::SIZE.z as u8 - 1),
                    Face::East => pos.with_x(0),
                    Face::West => pos.with_x(Chunk::SIZE.x as u8 - 1),
                };

                // SAFETY: values are hard-coded constants that should never be
                // out-of-bounds
                neighboring_chunk.block_at(unsafe { wrapped_pos.unwrap_unchecked() })
            }
            _ => false,
        }
    }

    #[must_use]
    pub const fn neighbor_from_face(&self, face: Face) -> Option<&ChunkData> {
        self.neighbors[face as usize]
    }
}
