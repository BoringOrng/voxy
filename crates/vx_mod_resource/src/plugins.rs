use bevy::app::{PluginGroup, PluginGroupBuilder};

use crate::{loader, registry};

#[derive(Default)]
pub struct Plugins;

impl PluginGroup for Plugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(registry::LoaderPlugin::<loader::TextureLoader>::default())
    }
}
