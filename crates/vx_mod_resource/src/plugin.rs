use std::fs;

use bevy::prelude::*;
use vx_mod::{LoadedMods, ModLoadState};

use crate::{TextureLoadState, TextureRegistry};

pub struct ModResourcePlugin;

impl ModResourcePlugin {
    #[expect(
        clippy::needless_pass_by_value,
        reason = "
            `Res<LoadedMods>` and `Res<AssetServer>` must be passed by value as
            is required by bevy
        "
    )]
    fn load_textures(
        loaded_mods: Res<LoadedMods>,
        asset_server: Res<AssetServer>,
        mut texture_registry: ResMut<TextureRegistry>,
        mut load_state: ResMut<NextState<TextureLoadState>>,
    ) {
        for loaded_mod in loaded_mods.iter() {
            let dir = loaded_mod.root().join("resource/texture");
            let Ok(entries) = fs::read_dir(&dir) else {
                info!(
                    "`{}` doesn't provide any texture data; skipping",
                    loaded_mod.id()
                );
                continue;
            };

            entries
                .filter_map(|res| {
                    res.inspect_err(|err| {
                        warn!(
                            "Failed to access a texture for `{}` ({err})",
                            loaded_mod.id()
                        );
                    })
                    .ok()
                })
                .filter(|entry| entry.file_name().to_string_lossy().ends_with(".png"))
                .map(|entry| {
                    (
                        entry
                            .file_name()
                            .to_string_lossy()
                            .split('.')
                            .next()
                            .unwrap()
                            .to_owned(),
                        asset_server.load(entry.path().strip_prefix("assets").unwrap().to_owned()),
                    )
                })
                .map(|(name, image)| (loaded_mod.id().to_owned() + "::texture::" + &name, image))
                .collect_into(&mut **texture_registry);
        }

        load_state.set(TextureLoadState::Loaded);
        info!("Loaded {} textures!", texture_registry.len());
    }
}

impl Plugin for ModResourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<TextureLoadState>()
            .init_resource::<TextureRegistry>()
            .add_systems(OnEnter(ModLoadState::Loaded), Self::load_textures);
    }
}
