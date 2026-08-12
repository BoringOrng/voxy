use std::{
    sync::Arc,
    time::{Duration, Instant},
};

use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task, futures},
};
use vx_mod_behavior::block::BlockRegistry;
use vx_world::{
    block::BlockPos,
    chunk::{Chunk, ChunkData, ChunkPos, chunk_state},
};

#[derive(Component)]
struct ChunkTask(Task<ChunkData>);

pub struct WorldgenPlugin;

impl WorldgenPlugin {
    const MAX_ACTIVE_TASKS: usize = 128;
    const HANDLE_TIME_BUDGET: Duration = Duration::from_millis(2);

    #[expect(
        clippy::needless_pass_by_value,
        reason = "
            `Res<BlockRegistry>` and `Res<vx_climate::Sampler>` must be passed by
            value as is required by bevy
        "
    )]
    fn generate_chunks(
        mut commands: Commands,
        block_registry: If<Res<BlockRegistry>>,
        climate_sampler: If<Res<vx_climate::Sampler>>,
        active_tasks: Query<(), With<ChunkTask>>,
        chunks: Query<(Entity, &ChunkPos), With<chunk_state::NeedsWorldgen>>,
    ) {
        let current_active = active_tasks.count();
        if current_active >= Self::MAX_ACTIVE_TASKS {
            return;
        }

        let spawn_budget = Self::MAX_ACTIVE_TASKS - current_active;

        let pool = AsyncComputeTaskPool::get();
        let block_registry = Arc::new(block_registry.clone());
        let climate_sampler = Arc::new(climate_sampler.clone());

        let tasks: Vec<_> = chunks
            .iter()
            .take(spawn_budget)
            .map(|(chunk_entity, &chunk_pos)| {
                let block_registry = block_registry.clone();
                let climate_sampler = climate_sampler.clone();

                let task = pool.spawn(async move {
                    Self::generate_chunk(chunk_pos, &block_registry, &climate_sampler)
                });

                (chunk_entity, ChunkTask(task))
            })
            .collect();

        if !tasks.is_empty() {
            commands.insert_batch(tasks);
        }
    }

    fn handle_finished_chunks(mut commands: Commands, mut tasks: Query<(Entity, &mut ChunkTask)>) {
        let now = Instant::now();

        for (chunk_entity, chunk_data) in tasks
            .iter_mut()
            .filter_map(|(task_entity, mut gen_task)| {
                futures::check_ready(&mut gen_task.0).map(|gen_data| (task_entity, gen_data))
            })
            .take_while(|_| now.elapsed() < Self::HANDLE_TIME_BUDGET)
        {
            commands
                .entity(chunk_entity)
                .remove::<(ChunkTask, chunk_state::NeedsWorldgen)>()
                .insert((chunk_state::NeedsMeshing, chunk_data));
        }
    }

    #[expect(
        clippy::cast_precision_loss,
        clippy::cast_possible_truncation,
        clippy::float_cmp,
        reason = "
            `Chunk::SIZE` shouldn't have any axis that exceeds 23-bit precision,
            nor 8-bit precision. Comparison of floats is valid as both have been
            truncated.
        "
    )]
    fn generate_chunk(
        chunk_pos: ChunkPos,
        block_registry: &BlockRegistry,
        climate_sampler: &vx_climate::Sampler,
    ) -> ChunkData {
        let mut chunk_data = ChunkData::default();

        let dirt_id = block_registry
            .get_id("voxy:dirt")
            .expect("dirt should exist");

        let grass_id = block_registry
            .get_id("voxy:grass")
            .expect("grass should exist");

        let world_base = chunk_pos.as_vec3() * Chunk::SIZE.as_vec3();

        for x in 0..Chunk::SIZE.x {
            for z in 0..Chunk::SIZE.z {
                let world_x = world_base.x + x as f32;
                let world_z = world_base.z + z as f32;

                // in range -1..=1
                let continental = climate_sampler.continental(Vec2::new(world_x, world_z));
                let surface_height = continental.trunc();

                if world_base.y > surface_height {
                    continue;
                }

                for y in 0..Chunk::SIZE.y {
                    let world_y = (world_base.y + y as f32).trunc();

                    if world_y > surface_height {
                        break;
                    }

                    let block = if world_y == surface_height {
                        grass_id
                    } else {
                        dirt_id
                    };

                    chunk_data.insert(BlockPos::new(x as u8, y as u8, z as u8), block);
                }
            }
        }

        chunk_data
    }
}

impl Plugin for WorldgenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (Self::generate_chunks, Self::handle_finished_chunks),
        );
    }
}
