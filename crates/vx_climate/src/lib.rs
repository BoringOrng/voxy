use bevy::prelude::*;

mod plugin;
mod sampler;

pub use plugin::ClimatePlugin;
pub use sampler::Sampler;

#[derive(Resource, Deref, Clone, Copy)]
#[component(immutable)]
pub struct WorldSeed(pub u32);
