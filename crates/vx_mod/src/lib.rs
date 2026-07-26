#![feature(iter_collect_into)]

mod load_state;
mod loaded_mods;
mod mod_info;
mod plugin;

pub use load_state::ModLoadState;
pub use loaded_mods::LoadedMods;
pub use mod_info::ModInfo;
pub use mod_manifest::ModManifest;
pub use plugin::CoreModPlugin;

pub mod mod_manifest;
