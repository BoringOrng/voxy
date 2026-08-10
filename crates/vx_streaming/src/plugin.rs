use std::sync::Arc;

use bevy::{
    math::bounding::Aabb3d, platform::collections::HashSet, prelude::*, tasks::AsyncComputeTaskPool,
};
use crossbeam_channel::{Receiver, Sender};
use vx_world::chunk::{Chunk, ChunkMap, ChunkPos, chunk_state};

pub struct StreamingPlugin;

#[derive(Resource)]
struct DespawnChannel {
    sender: Sender<Entity>,
    receiver: Receiver<Entity>,
}

#[derive(Resource)]
struct SpawnChannel {
    sender: Sender<ChunkPos>,
    receiver: Receiver<ChunkPos>,
}

impl StreamingPlugin {
    fn setup_channels(mut commands: Commands) {
        let (despawn_sender, despawn_receiver) = crossbeam_channel::unbounded();
        let (spawn_sender, spawn_receiver) = crossbeam_channel::unbounded();

        commands.insert_resource(DespawnChannel {
            sender: despawn_sender,
            receiver: despawn_receiver,
        });

        commands.insert_resource(SpawnChannel {
            sender: spawn_sender,
            receiver: spawn_receiver,
        });
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "
            `Res<DespawnChannel>`, `Res<ChunkMap>`, and `Res<super::Config>`
            must be passed by value as is required by bevy
        "
    )]
    fn start_despawn(
        channel: Res<DespawnChannel>,
        chunk_map: Res<ChunkMap>,
        stream_config: Res<super::Config>,
        anchor_positions: Query<&ChunkPos, (With<super::Anchor>, Changed<ChunkPos>)>,
    ) {
        let unload_radius = stream_config.unload_radius();
        let pool = AsyncComputeTaskPool::get();
        let chunk_map = Arc::new(chunk_map.clone());

        for &anchor_pos in &anchor_positions {
            let sender = channel.sender.clone();
            let chunk_map = chunk_map.clone();

            pool.spawn(async move {
                for &chunk in chunk_map
                    .keys()
                    .filter(|chunk_pos| chunk_pos.chebyshev_distance(*anchor_pos) >= unload_radius)
                    .filter_map(|chunk_pos| chunk_map.get(chunk_pos))
                {
                    _ = sender.send(chunk);
                }
            })
            .detach();
        }
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "`Res<DespawnChannel>` must be passed by value as is required by bevy"
    )]
    fn handle_despawn(channel: Res<DespawnChannel>, mut commands: Commands) {
        for msg in channel.receiver.try_iter() {
            // have to do try_despawn here because our pool might contain multiple of the
            // same entity
            commands.entity(msg).try_despawn();
        }
    }

    fn start_spawn(
        channel: Res<SpawnChannel>,
        chunk_map: Res<ChunkMap>,
        stream_config: Res<super::Config>,
        anchor_positions: Query<&ChunkPos, (With<super::Anchor>, Changed<ChunkPos>)>,
    ) {
        let load_radius = UVec3::splat(stream_config.load_radius());
        let pool = AsyncComputeTaskPool::get();
        let to_ignore = Arc::new(chunk_map.keys().copied().collect::<HashSet<_>>());

        for &anchor_pos in &anchor_positions {
            let spawn_bounds = Aabb3d::new(anchor_pos.as_vec3a(), load_radius.as_vec3a());
            let min = spawn_bounds.min.as_ivec3();
            let max = spawn_bounds.max.as_ivec3();

            let sender = channel.sender.clone();
            let to_ignore = to_ignore.clone();

            pool.spawn(async move {
                for x in min.x..=max.x {
                    for y in min.y..=max.y {
                        for z in min.z..=max.z {
                            let pos = ChunkPos::new(IVec3::new(x, y, z));
                            if to_ignore.contains(&pos) {
                                continue;
                            }

                            _ = sender.send(pos);
                        }
                    }
                }
            })
            .detach();
        }
    }

    fn handle_spawn(channel: Res<SpawnChannel>, chunk_map: Res<ChunkMap>, mut commands: Commands) {
        let chunk_size = Chunk::SIZE.as_vec3();
        let mut seen = HashSet::new();

        let to_spawn: Vec<_> = channel
            .receiver
            .try_iter()
            .filter(|chunk_pos| seen.insert(*chunk_pos) && !chunk_map.contains_key(chunk_pos))
            .map(|chunk_pos| {
                (
                    chunk_pos,
                    Transform::from_translation(chunk_pos.as_vec3() * chunk_size),
                    chunk_state::NeedsWorldgen,
                )
            })
            .collect();

        commands.spawn_batch(to_spawn);
    }
}

impl Plugin for StreamingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<super::Config>()
            .add_systems(Startup, Self::setup_channels)
            .add_systems(
                FixedUpdate,
                (
                    Self::start_despawn,
                    Self::handle_despawn,
                    Self::start_spawn,
                    Self::handle_spawn,
                )
                    .after(Self::setup_channels),
            );
    }
}
