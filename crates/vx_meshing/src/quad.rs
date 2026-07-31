use bevy::prelude::*;
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

    #[must_use]
    pub const fn uvs(self) -> [Vec2; 4] {
        Face::uv_corners()
    }

    #[must_use]
    pub fn positions(self) -> [Vec3; 4] {
        self.face().corners().map(|c| c + self.pos().as_vec3())
    }

    #[must_use]
    pub fn normals(self) -> [Vec3; 4] {
        [self.face().normal(); 4]
    }
}
