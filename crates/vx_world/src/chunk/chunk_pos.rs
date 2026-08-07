use bevy::prelude::*;

#[derive(Clone, Copy, Debug, Default, Deref, Component, PartialEq, Eq, Hash)]
pub struct ChunkPos(IVec3);

impl ChunkPos {
    #[must_use]
    pub fn from_vec3(pos: Vec3) -> Self {
        Self(pos.as_ivec3().div_euclid(super::Chunk::SIZE.as_ivec3()))
    }

    #[must_use]
    pub fn up(self) -> Self {
        Self(self.0 + IVec3::Y)
    }

    #[must_use]
    pub fn down(self) -> Self {
        Self(self.0 - IVec3::Y)
    }

    #[must_use]
    pub fn north(self) -> Self {
        Self(self.0 - IVec3::Z)
    }

    #[must_use]
    pub fn south(self) -> Self {
        Self(self.0 + IVec3::Z)
    }

    #[must_use]
    pub fn east(self) -> Self {
        Self(self.0 + IVec3::X)
    }

    #[must_use]
    pub fn west(self) -> Self {
        Self(self.0 - IVec3::X)
    }
}
