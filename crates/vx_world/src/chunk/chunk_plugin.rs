use bevy::prelude::*;

use crate::chunk::ChunkMap;

#[derive(Default)]
pub struct ChunkPlugin;

impl ChunkPlugin {}

impl Plugin for ChunkPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ChunkMap>();
    }
}
