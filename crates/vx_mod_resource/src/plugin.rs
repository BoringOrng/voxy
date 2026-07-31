use std::{
    fs::{self, DirEntry},
    path::Path,
};

use bevy::prelude::*;
use vx_mod::{LoadedMods, ModLoadState};

use crate::{TextureLoadState, TextureRegistry, block_array::BlockTextureArray};

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
                .filter_map(Result::ok)
                .filter_map(|entry| Self::try_load_texture(&asset_server, loaded_mod.id(), &entry))
                .collect_into(&mut **texture_registry);
        }

        load_state.set(TextureLoadState::Loaded);
        info!("Loaded {} textures!", texture_registry.len());
    }

    fn try_load_texture(
        asset_server: &AssetServer,
        mod_id: &str,
        entry: &DirEntry,
    ) -> Option<(String, Handle<Image>)> {
        let path = entry.path();

        if !Self::is_image(&path) {
            warn!("`{}` isn't an image", path.display());
            return None;
        }

        // logically, these shouldn't happen but whatever. Stripping the `assets` out of
        // the path could fail if mods are moved out of the `assets` directory,
        // currently this is done because the asset server must load there though, so
        // it's a bit of an iffy situation.
        let stem = path.file_stem()?.to_string_lossy();
        let asset_path = path.strip_prefix("assets").ok()?;

        Some((
            format!("{mod_id}::texture::{stem}"),
            asset_server.load(asset_path.to_owned()),
        ))
    }

    fn is_image(file_name: &Path) -> bool {
        const EXTENSIONS: &[&str] = &["png", "jpg", "jpeg"];
        EXTENSIONS
            .iter()
            .any(|ext| file_name.extension().is_some_and(|fext| fext == *ext))
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "
            `Res<LoadedMods>`, `Res<AssetServer>`, and `Res<Assets<Image>>` must
            be passed by value as is required by bevy
        "
    )]
    fn load_block_texture_array(
        texture_registry: Res<TextureRegistry>,
        asset_server: Res<AssetServer>,
        images: Res<Assets<Image>>,
        mut commands: Commands,
    ) {
        if texture_registry
            .values()
            .any(|img| !asset_server.is_loaded(img))
        {
            return;
        }

        commands.insert_resource(BlockTextureArray::build(
            &texture_registry,
            &asset_server,
            &images,
        ));
    }
}

impl Plugin for ModResourcePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<TextureLoadState>()
            .init_resource::<TextureRegistry>()
            .add_systems(OnEnter(ModLoadState::Loaded), Self::load_textures)
            .add_systems(
                Update,
                Self::load_block_texture_array.run_if(
                    in_state(TextureLoadState::Loaded)
                        .and_then(not(resource_exists::<BlockTextureArray>)),
                ),
            );
    }
}
