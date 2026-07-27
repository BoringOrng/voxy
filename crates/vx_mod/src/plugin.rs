use std::fs;

use bevy::prelude::*;

use crate::{LoadedMods, ModInfo, ModLoadState};

pub struct CoreModPlugin;

impl CoreModPlugin {
    pub fn load_mods(
        mut mods: ResMut<LoadedMods>,
        mut load_state: ResMut<NextState<ModLoadState>>,
    ) {
        let mods_dir = match fs::read_dir("assets/mods/") {
            Ok(mods_dir) => mods_dir,
            Err(err) => {
                warn!("Failed to load mods directory: ({err})");
                return;
            }
        };

        mods_dir
            .filter_map(Result::ok)
            .map(|entry| entry.path())
            .filter_map(|root| match ModInfo::load(root.clone()) {
                Ok(info) => {
                    info!("Successfully parsed mod `{}`!", info.id());
                    Some(info)
                }
                Err(err) => {
                    warn!("Couldn't load mod at `{root:?}`: {err}");
                    None
                }
            })
            .collect_into(&mut **mods);

        load_state.set(ModLoadState::Loaded);
    }
}

impl Plugin for CoreModPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ModLoadState>()
            .init_resource::<LoadedMods>()
            .add_systems(Startup, Self::load_mods);
    }
}
