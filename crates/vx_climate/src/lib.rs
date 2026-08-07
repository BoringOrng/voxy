use bevy::prelude::*;

mod plugin;

pub use plugin::ClimatePlugin;

#[derive(Resource, Deref)]
#[component(immutable)]
pub struct WorldSeed(pub u64);
