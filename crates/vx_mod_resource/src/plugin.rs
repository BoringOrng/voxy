use std::{
    fs::{self, DirEntry},
    path::Path,
};

use bevy::{platform::collections::HashMap, prelude::*};
use vx_mod::LoadedMods;

use crate::{TextureRegistry, block_array::BlockTextureArray};

pub struct ModResourcePlugin;

impl ModResourcePlugin {
    #[expect(
        clippy::needless_pass_by_value,
        reason = "
            `Res<LoadedMods>` and `Res<AssetServer>` must be passed by value as
            is required by bevy
        "
    )]
    fn setup_texture_registry(
        loaded_mods: Res<LoadedMods>,
        asset_server: Res<AssetServer>,
        mut commands: Commands,
    ) {
        let raw_texture_registry: HashMap<_, _> = loaded_mods
            .iter()
            .filter_map(|loaded_mod| {
                Some((
                    fs::read_dir(loaded_mod.root().join("resource/texture/")).ok()?,
                    loaded_mod.id(),
                ))
            })
            .flat_map(|(entries, mod_id)| {
                entries
                    .filter_map(Result::ok)
                    .filter_map(|entry| Self::try_load_texture(&asset_server, mod_id, &entry))
            })
            .collect();

        info!("Loaded {} textures!", raw_texture_registry.len());
        commands.insert_resource(TextureRegistry::new(raw_texture_registry));
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
    fn setup_block_texture_array(
        texture_registry: If<Res<TextureRegistry>>,
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

    fn invalidate_texture_array(mut commands: Commands) {
        commands.remove_resource::<BlockTextureArray>();
    }
}

impl Plugin for ModResourcePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            Self::setup_texture_registry.run_if(resource_exists_and_changed::<LoadedMods>),
        )
        .add_systems(
            Update,
            Self::invalidate_texture_array.run_if(resource_exists_and_changed::<TextureRegistry>),
        )
        .add_systems(
            Update,
            Self::setup_block_texture_array.run_if(not(resource_exists::<BlockTextureArray>)),
        );
    }
}
