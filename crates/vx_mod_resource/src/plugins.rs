use bevy::{
    app::PluginGroupBuilder,
    asset::io::{AssetSourceBuilder, file::FileAssetReader},
    prelude::*,
};

use crate::{loader, registry};

#[derive(Default)]
pub struct Plugins;

impl PluginGroup for Plugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(ModsAssetSourcePlugin)
            .add(registry::LoaderPlugin::<loader::TextureLoader>::default())
    }
}

#[derive(Default)]
struct ModsAssetSourcePlugin;

impl Plugin for ModsAssetSourcePlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_source(
            "mods",
            AssetSourceBuilder::new(|| Box::new(FileAssetReader::new("mods"))),
        );
    }
}
