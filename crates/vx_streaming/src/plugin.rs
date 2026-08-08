use bevy::{math::bounding::Aabb3d, platform::collections::HashSet, prelude::*};
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

    #[expect(
        clippy::needless_pass_by_value,
        reason = "`Res<ChunkMap>` and `Res<super::Config>` must be passed by value as is required by bevy"
    )]
    fn spawn_chunks(
        mut commands: Commands,
        chunk_map: Res<ChunkMap>,
        stream_config: Res<super::Config>,
        anchor_positions: Query<&ChunkPos, With<super::Anchor>>,
    ) {
        let spawn_radius = stream_config.load_radius();
        let to_ignore: &HashSet<_> = &chunk_map.keys().copied().collect();

        let to_spawn: Vec<_> = anchor_positions
            .iter()
            .flat_map(|&anchor_pos| {
                let spawn_bounds =
                    Aabb3d::new(anchor_pos.as_vec3a(), Vec3A::splat(spawn_radius as f32));

                let min = spawn_bounds.min.as_ivec3();
                let max = spawn_bounds.max.as_ivec3();

                (min.x..=max.x)
                    // two flat maps here can be read as
                    // `for x in .. { for y in .. { for z in .. { (x, y, z) } } }`
                    .flat_map(move |x| (min.y..=max.y).map(move |y| (x, y)))
                    .flat_map(move |(x, y)| (min.z..=max.z).map(move |z| (x, y, z)))
                    .map(|(x, y, z)| ChunkPos::new(IVec3::new(x, y, z)))
                    .filter(move |pos| !to_ignore.contains(pos))
            })
            .map(|chunk_pos| (chunk_pos, super::PendingGeneration))
            .collect();

        commands.spawn_batch(to_spawn);
    }
}

impl Plugin for StreamingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<super::Config>()
            .add_systems(FixedUpdate, (Self::despawn_chunks, Self::spawn_chunks));
    }
}
