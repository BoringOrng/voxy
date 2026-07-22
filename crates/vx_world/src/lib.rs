use bevy::app::plugin_group;

pub mod block;
pub mod chunk;

plugin_group! {
    #[derive(Default)]
    pub struct WorldPlugins {
        chunk:::ChunkPlugin,
    }
}
