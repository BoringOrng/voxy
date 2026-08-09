use bevy::prelude::*;
use vx_mod_behavior::block::BlockRegistry;
use vx_world::{
    block::BlockPos,
    chunk::{ChunkData, ChunkPos, chunk_state},
};

pub struct WorldgenPlugin;

impl WorldgenPlugin {
    fn generate_chunks(
        mut commands: Commands,
        block_registry: Res<BlockRegistry>,
        chunks: Query<(Entity, &ChunkPos), With<chunk_state::NeedsWorldgen>>,
    ) {
        for (chunk_entity, chunk_pos) in &chunks {
            let mut chunk_commands = commands.entity(chunk_entity);

            match chunk_pos.y {
                y if y > 0 => {}
                y if y < 0 => {}
                _ => {
                    let mut chunk_data = ChunkData::default();
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

                    chunk_commands.insert(chunk_data);
                }
            }

            chunk_commands
                .remove::<chunk_state::NeedsWorldgen>()
                .insert(chunk_state::NeedsMeshing);
        }
    }
}

impl Plugin for WorldgenPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, Self::generate_chunks);
    }
}
