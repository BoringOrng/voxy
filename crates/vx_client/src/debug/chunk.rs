use bevy::{color::palettes::css, prelude::*};
use vx_world::chunk::{Chunk, ChunkPos, chunk_state};

#[derive(Default)]
pub struct ChunkDebugPlugin;

type GeneratedChunk = (
    With<Chunk>,
    Without<chunk_state::NeedsWorldgen>,
    Without<chunk_state::NeedsMeshing>,
);

impl ChunkDebugPlugin {
    fn draw_chunk_bounds(
        mut gizmos: Gizmos,
        pending: Query<&ChunkPos, With<chunk_state::NeedsWorldgen>>,
        dirty: Query<&ChunkPos, With<chunk_state::NeedsMeshing>>,
        ready: Query<&ChunkPos, GeneratedChunk>,
    ) {
        for &pos in &pending {
            Self::draw_chunk(&mut gizmos, pos, css::ORANGE.into());
        }

        for &pos in &dirty {
            Self::draw_chunk(&mut gizmos, pos, css::YELLOW.into());
        }

        for &pos in &ready {
            Self::draw_chunk(&mut gizmos, pos, css::LIME.into());
        }
    }

    fn draw_chunk(gizmos: &mut Gizmos, pos: ChunkPos, color: Color) {
        let size = Chunk::SIZE.as_vec3();
        let center = pos.as_vec3() * size + size * 0.5;

        gizmos.cube(Transform::from_translation(center).with_scale(size), color);
    }
}

impl Plugin for ChunkDebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, Self::draw_chunk_bounds);
    }
}
