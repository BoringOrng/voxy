use bevy::{
    ecs::{lifecycle::HookContext, world::DeferredWorld},
    math::USizeVec3,
    prelude::*,
};

mod chunk_data;
mod chunk_map;
mod chunk_plugin;
mod chunk_pos;

pub use chunk_data::ChunkData;
pub use chunk_map::ChunkMap;
pub use chunk_plugin::ChunkPlugin;
pub use chunk_pos::ChunkPos;

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash, Component)]
#[require(ChunkData, ChunkPos, Transform)]
#[component(on_add, on_remove)]
pub struct Chunk;

impl Chunk {
    pub const SIZE: USizeVec3 = USizeVec3::splat(32);
    pub const VOLUME: usize = 32 * 32 * 32;

    fn on_add(mut world: DeferredWorld, ctx: HookContext) {
        let pos = *world
            .get::<ChunkPos>(ctx.entity)
            .expect("all chunks should have a ChunkPos");

        let mut chunk_map = world.resource_mut::<ChunkMap>();

        chunk_map.insert(pos, ctx.entity);
    }

    fn on_remove(mut world: DeferredWorld, ctx: HookContext) {
        let pos = *world
            .get::<ChunkPos>(ctx.entity)
            .expect("all chunks should have a ChunkPos");

        let mut chunk_map = world.resource_mut::<ChunkMap>();

        chunk_map.remove(&pos);
    }
}
