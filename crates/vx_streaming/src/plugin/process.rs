use bevy::prelude::*;
use vx_world::chunk::{Chunk, ChunkMap, chunk_state};

use super::StreamingPlugin;

#[expect(
    clippy::needless_pass_by_value,
    reason = "`Res<ChunkMap>` must be passed by value as is required by bevy"
)]
pub fn process_spawn_queue(
    mut commands: Commands,
    mut spawn_queue: ResMut<crate::Queue>,
    worldgen_queued: Query<(), With<chunk_state::NeedsWorldgen>>,
    chunk_map: Res<ChunkMap>,
) {
    let in_queue = worldgen_queued.iter().count();
    let budget = StreamingPlugin::MAX_ACTIVE_SPAWNS.saturating_sub(in_queue);
    if budget == 0 {
        return;
    }

    let chunk_size = Chunk::SIZE.as_vec3();

    let to_spawn: Vec<_> = spawn_queue
        .pop_batch(budget, &chunk_map)
        .into_iter()
        .map(|pos| {
            (
                pos,
                Transform::from_translation(pos.as_vec3() * chunk_size),
                chunk_state::NeedsWorldgen,
            )
        })
        .collect();

    if !to_spawn.is_empty() {
        commands.spawn_batch(to_spawn);
    }
}
