pub mod registry;

mod loader;
mod plugins;

pub(crate) use loader::Loader;

mod block_array;
mod plugin;
mod texture_map;

pub use block_array::BlockTextureArray;
pub use plugin::ModResourcePlugin;
pub use plugins::Plugins;
pub use registry::Registry;
pub use texture_map::TextureRegistry;
