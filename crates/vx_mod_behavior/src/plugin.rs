use std::{fs, path::Path};

use bevy::{platform::collections::HashMap, prelude::*};
use vx_mod::{LoadedMods, ModLoadState};

use crate::{
    block::{Block, BlockLoadState, BlockRegistry},
    error::Error,
};

pub struct ModBehaviorPlugin;

impl ModBehaviorPlugin {
    #[expect(
        clippy::needless_pass_by_value,
        reason = "`Res<LoadedMods>` must be passed by value as is required by bevy"
    )]
    pub fn load_blocks(
        loaded_mods: Res<LoadedMods>,
        mut block_registry: ResMut<BlockRegistry>,
        mut load_state: ResMut<NextState<BlockLoadState>>,
    ) {
        let mut raw_block_registry = HashMap::new();

        for loaded_mod in loaded_mods.iter() {
            let dir = loaded_mod.root().join("behavior/block/");
            let Ok(entries) = fs::read_dir(&dir) else {
                continue;
            };

            entries
                .filter_map(Result::ok)
                .map(|entry| entry.path())
                .filter_map(|path| {
                    Self::try_load_block(&path)
                        .inspect_err(|err| warn!("{err}"))
                        .ok()
                })
                .collect_into(&mut raw_block_registry);
        }

        *block_registry = BlockRegistry::build(raw_block_registry);

        load_state.set(BlockLoadState::Loaded);
        info!("Loaded {} blocks!", block_registry.len());
    }

    fn try_load_block(path: &Path) -> Result<(String, Block), Error> {
        let block_data = fs::read_to_string(path)?;
        let parsed: Block = ron::from_str(&block_data)?;

        Ok((parsed.id().to_owned(), parsed))
    }
}

impl Plugin for ModBehaviorPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<BlockLoadState>()
            .init_resource::<BlockRegistry>()
            .add_systems(OnEnter(ModLoadState::Loaded), Self::load_blocks);
    }
}
