use bevy::prelude::*;

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
            Self::North => IVec3::Z,
            Self::South => IVec3::NEG_Z,
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
            Self::Up => c![3, 7, 6, 2],
            Self::Down => c![4, 0, 1, 5],
            Self::North => c![4, 5, 6, 7],
            Self::South => c![1, 0, 3, 2],
            Self::East => c![1, 2, 6, 5],
            Self::West => c![4, 7, 3, 0],
        }
    }
}
