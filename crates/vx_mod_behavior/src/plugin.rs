use std::fs;

use bevy::prelude::*;
use vx_mod::{LoadedMods, ModLoadState};

use crate::block::{Block, BlockLoadState, BlockRegistry};

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
        for m in loaded_mods.iter() {
            let dir = m.root().join("behavior/block/");
            let Ok(entries) = fs::read_dir(&dir) else {
                info!("`{}` doesn't provide any block data; skipping", m.id());
                continue;
            };

            entries
                .filter_map(|res| {
                    res.inspect_err(|err| {
                        warn!(
                            "Failed to access a piece of block data for `{}` ({err})",
                            m.id()
                        );
                    })
                    .ok()
                })
                .filter_map(|entry| {
                    fs::read_to_string(entry.path())
                        .inspect_err(|err| {
                            warn!("Failed to read block data at `{:?}` ({err})", entry.path());
                        })
                        .map(|text| (entry, text))
                        .ok()
                })
                .filter_map(|(entry, text)| {
                    ron::from_str::<Block>(&text)
                        .inspect_err(|err| {
                            warn!("Failed to parse block data at `{:?}` ({err})", entry.path());
                        })
                        .ok()
                })
                .map(|block| (block.id().to_owned(), block))
                .collect_into(&mut **block_registry);
        }

        load_state.set(BlockLoadState::Loaded);
        info!("All blocks loaded!");
    }
}

impl Plugin for ModBehaviorPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<BlockLoadState>()
            .init_resource::<BlockRegistry>()
            .add_systems(OnEnter(ModLoadState::Loaded), Self::load_blocks);
    }
}
