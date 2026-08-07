use bevy::prelude::*;
use vx_world::chunk::{ChunkMap, ChunkPos};

pub struct StreamingPlugin;

impl StreamingPlugin {
    #[expect(
        clippy::needless_pass_by_value,
        reason = "`Res<super::Config>` must be passed by value as is required by bevy"
    )]
    fn despawn_chunks(
        mut chunk_map: ResMut<ChunkMap>,
        mut commands: Commands,
        stream_config: Res<super::Config>,
        anchor_positions: Query<&ChunkPos, With<super::Anchor>>,
    ) {
        let drop_dist = stream_config.unload_radius();

        let to_drop: Vec<_> = anchor_positions
            .iter_inner()
            .flat_map(|&anchor_pos| {
                chunk_map
                    .keys()
                    .filter(move |chunk_pos| chunk_pos.chebyshev_distance(*anchor_pos) >= drop_dist)
                    .copied()
            })
            .collect();

        for drop_pos in to_drop {
            let chunk_entity = chunk_map.remove(&drop_pos).unwrap();
            commands.entity(chunk_entity).despawn();

            info!("dropped chunk at {drop_pos:?}");
        }
    }
}

impl Plugin for StreamingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<super::Config>()
            .add_systems(FixedUpdate, Self::despawn_chunks);
    }
}
