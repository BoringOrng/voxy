use bevy::prelude::*;
use vx_world::{block::BlockPos, chunk::Chunk};

#[derive(Clone, Copy)]
pub enum Face {
    Up,
    Down,
    North,
    South,
    East,
    West,
}

impl Face {
    pub const ALL: [Self; 6] = [
        Self::Up,
        Self::Down,
        Self::North,
        Self::South,
        Self::East,
        Self::West,
    ];

    #[must_use]
    pub const fn offset(self) -> IVec3 {
        match self {
            Self::Up => IVec3::Y,
            Self::Down => IVec3::NEG_Y,
            Self::North => IVec3::NEG_Z,
            Self::South => IVec3::Z,
            Self::East => IVec3::X,
            Self::West => IVec3::NEG_X,
        }
    }

    #[must_use]
    pub fn normal(self) -> Vec3 {
        self.offset().as_vec3()
    }

    #[must_use]
    pub const fn corners(self) -> [Vec3; 4] {
        macro_rules! c { [$($i: literal),*] => { [$(CORNERS[$i]),*] }; }
        const CORNERS: [Vec3; 8] = [
            Vec3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            Vec3::new(1.0, 1.0, 0.0),
            Vec3::new(0.0, 1.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            Vec3::new(1.0, 0.0, 1.0),
            Vec3::new(1.0, 1.0, 1.0),
            Vec3::new(0.0, 1.0, 1.0),
        ];

        match self {
            Self::Up => c![2, 3, 7, 6],
            Self::Down => c![0, 1, 5, 4],
            Self::North => c![3, 2, 1, 0],
            Self::South => c![6, 7, 4, 5],
            Self::East => c![2, 6, 5, 1],
            Self::West => c![7, 3, 0, 4],
        }
    }

    #[expect(
        clippy::cast_possible_truncation,
        reason = "`Chunk::SIZE.*` should always fit within an `u8`"
    )]
    #[must_use]
    pub const fn wrap_pos(self, pos: BlockPos) -> BlockPos {
        let wrapped = match self {
            Self::Up => pos.with_y(0),
            Self::Down => pos.with_y(Chunk::SIZE.y as u8 - 1),
            Self::North => pos.with_z(Chunk::SIZE.z as u8 - 1),
            Self::South => pos.with_z(0),
            Self::East => pos.with_x(0),
            Self::West => pos.with_x(Chunk::SIZE.x as u8 - 1),
        };

        // SAFETY: hard-coded values shouldn't be outside the bounds of a `BlockPos`
        unsafe { wrapped.unwrap_unchecked() }
    }

    #[must_use]
    pub const fn offset_pos(self, pos: BlockPos) -> Option<BlockPos> {
        match self {
            Self::Up => pos.up(),
            Self::Down => pos.down(),
            Self::North => pos.north(),
            Self::South => pos.south(),
            Self::East => pos.east(),
            Self::West => pos.west(),
        }
    }
}
