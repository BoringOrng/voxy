use std::{fs, path::Path};

use bevy::prelude::*;
use vx_mod::LoadedMods;

use crate::{
    block::{Block, BlockRegistry},
    error::Error,
};

pub struct ModBehaviorPlugin;

impl ModBehaviorPlugin {
    #[expect(
        clippy::needless_pass_by_value,
        reason = "`Res<LoadedMods>` must be passed by value as is required by bevy"
    )]
    pub fn setup_block_registry(loaded_mods: Res<LoadedMods>, mut commands: Commands) {
        let raw_block_registry = loaded_mods
            .iter()
            .filter_map(|loaded_mod| fs::read_dir(loaded_mod.behavior_path().join("block/")).ok())
            .flat_map(|entries| {
                entries.filter_map(|entry| {
                    Self::try_load_block(&entry.ok()?.path())
                        .inspect_err(|err| warn!("{err}"))
                        .ok()
                })
            })
            .collect();

        let block_registry = BlockRegistry::build(raw_block_registry);
        info!("Loaded {} blocks!", block_registry.len());

        commands.insert_resource(block_registry);
    }

    fn try_load_block(path: &Path) -> Result<(String, Block), Error> {
        let block_data = fs::read_to_string(path)?;
        let parsed: Block = ron::from_str(&block_data)?;

        Ok((parsed.id().to_owned(), parsed))
    }
}

impl Plugin for ModBehaviorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            Self::setup_block_registry.run_if(resource_exists_and_changed::<LoadedMods>),
        );
    }
}
