use std::path::Path;

use bevy::prelude::*;

mod texture;

pub use texture::TextureLoader;

pub trait Loader: Sized + Send + Sync + 'static {
    type Asset: Send + Sync + 'static;
    type Error: std::error::Error;

    // not the dir from the mods root, it's the dir from the `mod/resources` root.
    const DIR: &'static str;

    fn extensions() -> &'static [&'static str];

    /// # Errors
    ///
    /// Returns the associated [`Loader::Error`] when loading the asset fails.
    fn try_load(path: &str, asset_server: &AssetServer) -> Result<Self::Asset, Self::Error>;

    // anyone looking to implement this should take note that this is *only* true
    // for synchronous asset loading. Things that are asynchronous should override
    // this method.
    fn is_ready(asset: &Self::Asset, asset_server: &AssetServer) -> bool {
        _ = (asset, asset_server);

        true
    }

    #[must_use]
    fn matches_extension(path: &Path) -> bool {
        Self::extensions()
            .iter()
            .any(|ext| path.extension().is_some_and(|e| e == *ext))
    }
}
