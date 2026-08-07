use std::{fs, marker::PhantomData, path::Path};

use bevy::{platform::collections::HashMap, prelude::*};
use vx_mod::LoadedMods;

pub struct LoaderPlugin<L>(PhantomData<L>);

impl<L> Default for LoaderPlugin<L> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<L: crate::Loader> LoaderPlugin<L> {
    #[expect(
        clippy::needless_pass_by_value,
        reason = "`Res<LoadedMods>` must be passed by value as is required by bevy"
    )]
    fn setup_registry(
        loaded_mods: Res<LoadedMods>,
        asset_server: Res<AssetServer>,
        mut commands: Commands,
    ) {
        let asset_server = &asset_server.into_inner();

        let registry: HashMap<_, _> = loaded_mods
            .iter()
            .filter_map(|m| {
                let dir = m.root().join("resource").join(L::DIR);
                Some((fs::read_dir(dir).ok()?, m.id()))
            })
            .flat_map(|(entries, mod_id)| {
                entries
                    .filter_map(Result::ok)
                    .map(|entry| entry.path())
                    .filter(|path| L::matches_extension(path))
                    .filter_map(|path| {
                        Self::try_load_entry(asset_server, mod_id, &path)
                            .inspect_err(|err| warn!("Couldn't load `{}`: {err}", path.display()))
                            .ok()
                    })
            })
            .collect();

        info!("Loading {} entries in `{}`", registry.len(), L::DIR);

        commands.remove_resource::<super::Ready<L>>();
        commands.insert_resource(super::Registry::<L>::new(registry));
    }

    fn try_load_entry(
        asset_server: &AssetServer,
        mod_id: &str,
        path: &Path,
    ) -> Result<(String, L::Asset), L::Error> {
        let stem = path.file_stem().unwrap_or_default().to_string_lossy();
        let asset = L::try_load(path, asset_server)?;

        Ok((format!("{mod_id}::{}::{stem}", L::DIR), asset))
    }

    #[expect(
        clippy::needless_pass_by_value,
        reason = "
            `If<Res<super::Registry<L>>>` and `Res<AssetServer>` must be passed
            by value as is required by bevy
        "
    )]
    fn mark_ready(
        registry: If<Res<super::Registry<L>>>,
        asset_server: Res<AssetServer>,
        mut commands: Commands,
    ) {
        if registry.values().all(|a| L::is_ready(a, &asset_server)) {
            commands.insert_resource(super::Ready::<L>::default());
        }
    }
}

impl<L: crate::Loader> Plugin for LoaderPlugin<L> {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            Self::setup_registry.run_if(resource_exists_and_changed::<LoadedMods>),
        )
        .add_systems(
            Update,
            Self::mark_ready.run_if(
                resource_exists::<super::Registry<L>>
                    .or_else(not(resource_exists::<super::Ready<L>>)),
            ),
        );
    }
}
