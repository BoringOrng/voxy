#![feature(iter_collect_into)]

mod load_state;
mod plugin;
mod texture_map;

pub use load_state::TextureLoadState;
pub use plugin::ModResourcePlugin;
pub use texture_map::TextureRegistry;
