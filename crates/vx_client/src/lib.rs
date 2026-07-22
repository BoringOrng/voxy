use bevy::{app::PluginGroupBuilder, prelude::*};

pub mod player;

use player::LocalPlayerPlugin;

#[derive(Default)]
pub struct VxClientPlugins;

impl PluginGroup for VxClientPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add_group(DefaultPlugins)
            .add(LocalPlayerPlugin::default())
    }
}
