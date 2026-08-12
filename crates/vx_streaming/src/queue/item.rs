use bevy::math::{Dir3, FloatPow as _, Vec3};
use ordered_float::NotNan;
use vx_world::chunk::ChunkPos;

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Item {
    dist: NotNan<f32>,
    pos: ChunkPos,
}

impl Item {
    pub fn new(anchor_pos: ChunkPos, forward_dir: Dir3, chunk_pos: ChunkPos) -> Self {
        Self {
            dist: Self::score(anchor_pos, forward_dir, chunk_pos),
            pos: chunk_pos,
        }
    }

    #[must_use]
    pub const fn pos(self) -> ChunkPos {
        self.pos
    }

    #[must_use]
    fn score(anchor_pos: ChunkPos, forward_dir: Dir3, chunk_pos: ChunkPos) -> NotNan<f32> {
        let anchor = anchor_pos.as_vec3();
        let target = chunk_pos.as_vec3();

        let raw = target - anchor + Vec3::splat(0.5);
        let weighted = raw.with_y(raw.y * 0.02);

        let (dir, len) = weighted.normalize_and_length();
        let dot = forward_dir.dot(dir).squared();

        // SAFETY: adding `Vec3::splat(0.5)` guarantees we never normalize zero
        unsafe { NotNan::new_unchecked(len.recip().mul_add(10.0, dot)) }
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.dist.cmp(&other.dist)
    }
}
impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}
