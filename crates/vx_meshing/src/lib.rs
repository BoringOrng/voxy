use bevy::app::plugin_group;

pub mod block;
pub mod chunk;
mod quad;

pub use quad::Quad;

plugin_group! {
    pub struct MeshingPlugins {
        block:::BlockMeshingPlugin,
        chunk:::ChunkMeshingPlugin,
    }
}
