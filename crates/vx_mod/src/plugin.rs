use std::fs;

use bevy::prelude::*;

use crate::{LoadedMods, ModInfo, ModLoadState, ModManifest};

pub struct CoreModPlugin;

impl CoreModPlugin {
    pub fn load_mods(
        mut mods: ResMut<LoadedMods>,
        mut load_state: ResMut<NextState<ModLoadState>>,
    ) {
        let mods_dir = match fs::read_dir("assets/mods/") {
            Ok(mods_dir) => mods_dir,
            Err(err) => {
                warn!("Failed to load mods directory ({err})");
                return;
            }
        };

        mods_dir
            .filter_map(|res| {
                res.inspect_err(|err| {
                    warn!("couldn't open a mod directory ({err})");
                })
                .ok()
            })
            .map(|entry| entry.path())
            .filter(|p| {
                let manifest_exists = p.join("mod.toml").exists();

                if !manifest_exists {
                    warn!("Couldn't load mod at `{p:?}`, it doesn't have a `mod.toml`");
                }

                manifest_exists
            })
            .filter_map(|root| {
                let manifest = fs::read_to_string(root.join("mod.toml"))
                    .inspect_err(|err| {
                        warn!(
                            "Couldn't load mod at `{root:?}`, failure to load `mod.toml` ({err})"
                        );
                    })
                    .ok()?;

                let parsed: ModManifest = toml::from_str(&manifest)
                    .inspect_err(|err| {
                        warn!(
                            "Couldn't load mod at `{root:?}`, failure to parse `mod.toml` ({err})"
                        );
                    })
                    .ok()?;

                let package_name = parsed.package().name();
                info!("Successfully parsed mod `{package_name}`!");

                Some(ModInfo::new(package_name.to_owned(), root))
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
