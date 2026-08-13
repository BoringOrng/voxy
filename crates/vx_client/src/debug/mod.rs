mod chunk;

use bevy::app::plugin_group;

plugin_group! {
    pub struct Plugins {
        chunk:::ChunkDebugPlugin,
    }
}
