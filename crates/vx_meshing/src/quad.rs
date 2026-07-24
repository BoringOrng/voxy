use vx_world::block::BlockPos;

use crate::block::Face;

#[derive(Clone, Copy)]
pub struct Quad {
    pos: BlockPos,
    face: Face,
}

impl Quad {
    #[must_use]
    pub const fn new(pos: BlockPos, face: Face) -> Self {
        Self { pos, face }
    }

    #[must_use]
    pub const fn pos(self) -> BlockPos {
        self.pos
    }

    #[must_use]
    pub const fn face(self) -> Face {
        self.face
    }
}
