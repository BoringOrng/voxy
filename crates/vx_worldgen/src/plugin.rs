use bevy::{
    prelude::*,
    tasks::{AsyncComputeTaskPool, Task, futures},
};
use vx_mod_behavior::block::BlockRegistry;
use vx_world::{
    block::BlockPos,
    chunk::{ChunkData, ChunkPos, chunk_state},
};

#[derive(Component)]
struct ChunkTask(Task<ChunkData>);

pub struct WorldgenPlugin;

impl WorldgenPlugin {
    fn generate_chunks(
        mut commands: Commands,
        block_registry: Res<BlockRegistry>,
        chunks: Query<(Entity, &ChunkPos), With<chunk_state::NeedsWorldgen>>,
    ) {
        let pool = AsyncComputeTaskPool::get();

        for (chunk_entity, &chunk_pos) in &chunks {
            // horrendous but this should be cleaned up eventually anyway
            let block_registry = block_registry.clone();

            let task = pool.spawn(async move { Self::generate_chunk(chunk_pos, &block_registry) });
            commands.entity(chunk_entity).insert(ChunkTask(task));
        }
    }

    fn handle_finished_chunks(mut commands: Commands, mut tasks: Query<(Entity, &mut ChunkTask)>) {
        for (chunk_entity, chunk_data) in
            tasks.iter_mut().filter_map(|(task_entity, mut gen_task)| {
                futures::check_ready(&mut gen_task.0).map(|gen_data| (task_entity, gen_data))
            })
        {
            commands
                .entity(chunk_entity)
                .remove::<ChunkTask>()
                .insert(chunk_state::NeedsMeshing)
                .remove::<chunk_state::NeedsWorldgen>()
                .insert(chunk_data);
        }
    }

    fn generate_chunk(pos: ChunkPos, block_registry: &BlockRegistry) -> ChunkData {
        let mut chunk_data = ChunkData::default();

        match pos.y {
            y if y > 0 => {}
            y if y < 0 => {}
            _ => {
                let dirt_id = block_registry
                    .get_id("voxy:dirt")
                    .expect("dirt should exist");

                let grass_id = block_registry
                    .get_id("voxy:grass")
                    .expect("grass should exist");

                for x in 0..32 {
                    for y in 0..2 {
                        for z in 0..32 {
                            chunk_data.insert(BlockPos::new(x, y, z), dirt_id);
                        }
                    }
                }

                for x in 0..32 {
                    for z in 0..32 {
                        chunk_data.insert(BlockPos::new(x, 2, z), grass_id);
                    }
                }
            }
        }

        chunk_data
    }
}

impl Plugin for WorldgenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            FixedUpdate,
            (Self::generate_chunks, Self::handle_finished_chunks),
        );
    }
}
