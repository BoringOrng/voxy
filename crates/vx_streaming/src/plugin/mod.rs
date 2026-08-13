use bevy::prelude::*;

mod discover;
mod process;

pub struct StreamingPlugin;

impl StreamingPlugin {
    const MAX_ACTIVE_SPAWNS: usize = 128;
}

impl Plugin for StreamingPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<super::Config>()
            .init_resource::<super::Queue>()
            .add_systems(FixedPreUpdate, discover::discover_chunks)
            .add_systems(Update, process::process_spawn_queue);
    }
}
