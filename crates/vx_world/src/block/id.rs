use std::num::NonZeroU16;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BlockId(NonZeroU16);

impl BlockId {
    #[must_use]
    pub const fn from_raw(v: NonZeroU16) -> Self {
        Self(v)
    }

    #[must_use]
    pub const fn raw(self) -> NonZeroU16 {
        self.0
    }
}
