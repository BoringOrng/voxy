use std::fs;

use bevy::prelude::*;

use crate::{LoadedMods, ModInfo};

pub struct CoreModPlugin;

impl CoreModPlugin {
    pub fn load_mods(mut commands: Commands) {
        let mods_dir = match fs::read_dir("assets/mods/") {
            Ok(mods_dir) => mods_dir,
            Err(err) => {
                warn!("Failed to load mods directory: ({err})");
                return;
            }
        };

        let mods = mods_dir
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
            .collect();

        commands.insert_resource(LoadedMods::new(mods));
    }
}

impl Plugin for CoreModPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, Self::load_mods);
    }
}
