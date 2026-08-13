use bevy::math::{U8Vec3, Vec3};

#[derive(Clone, Copy)]
pub struct BlockPos(u16);

impl BlockPos {
    const MASK: u16 = 0x1f;

    #[must_use]
    pub const fn new(x: u8, y: u8, z: u8) -> Self {
        Self(Self::compress(x, y, z))
    }

    #[must_use]
    pub const fn up(self) -> Option<Self> {
        self.with_y(self.y() + 1)
    }

    #[must_use]
    pub const fn down(self) -> Option<Self> {
        // fun little hack, this overflows to u8::MAX which cant fit so it fails
        self.with_y(self.y().wrapping_sub(1))
    }

    #[must_use]
    pub const fn north(self) -> Option<Self> {
        // fun little hack, this overflows to u8::MAX which cant fit so it fails
        self.with_z(self.z().wrapping_sub(1))
    }

    #[must_use]
    pub const fn south(self) -> Option<Self> {
        self.with_z(self.z() + 1)
    }

    #[must_use]
    pub const fn east(self) -> Option<Self> {
        self.with_x(self.x() + 1)
    }

    #[must_use]
    pub const fn west(self) -> Option<Self> {
        // fun little hack, this overflows to u8::MAX which cant fit so it fails
        self.with_x(self.x().wrapping_sub(1))
    }

    #[must_use]
    pub const fn can_fit(v: u8) -> bool {
        v < (1 << Self::MASK.bit_width())
    }

    #[must_use]
    pub const fn from_raw(v: u16) -> Self {
        Self(v)
    }

    #[must_use]
    pub const fn raw(self) -> u16 {
        self.0
    }

    #[must_use]
    pub const fn as_u8vec3(self) -> U8Vec3 {
        U8Vec3::new(self.x(), self.y(), self.z())
    }

    #[must_use]
    pub const fn as_vec3(self) -> Vec3 {
        Vec3::new(self.x() as f32, self.y() as f32, self.z() as f32)
    }

    #[must_use]
    pub const fn x(self) -> u8 {
        (self.0 & Self::MASK) as u8
    }

    #[must_use]
    pub const fn y(self) -> u8 {
        ((self.0 >> 5) & Self::MASK) as u8
    }

    #[must_use]
    pub const fn z(self) -> u8 {
        ((self.0 >> 10) & Self::MASK) as u8
    }

    #[must_use]
    pub const fn with_x(self, x: u8) -> Option<Self> {
        if !Self::can_fit(x) {
            return None;
        }

        Some(Self::new(x, self.y(), self.z()))
    }

    #[must_use]
    pub const fn with_y(self, y: u8) -> Option<Self> {
        if !Self::can_fit(y) {
            return None;
        }

        Some(Self::new(self.x(), y, self.z()))
    }

    #[must_use]
    pub const fn with_z(self, z: u8) -> Option<Self> {
        if !Self::can_fit(z) {
            return None;
        }

        Some(Self::new(self.x(), self.y(), z))
    }

    const fn compress(x: u8, y: u8, z: u8) -> u16 {
        (x as u16 & Self::MASK) | (y as u16 & Self::MASK) << 5 | (z as u16 & Self::MASK) << 10
    }
}
