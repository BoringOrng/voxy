mod error;

use bevy::prelude::*;
use error::Error;

pub struct TextureLoader;

impl super::Loader for TextureLoader {
    type Asset = Handle<Image>;
    type Error = Error;

    const DIR: &'static str = "texture";

    fn extensions() -> &'static [&'static str] {
        &["png", "jpg", "jpeg"]
    }

    fn try_load(
        path: &std::path::Path,
        asset_server: &AssetServer,
    ) -> Result<Self::Asset, Self::Error> {
        let asset_path = path
            .strip_prefix("assets")
            .map_err(|_| Error::OutsideAssets(path.to_owned()))?;

        Ok(asset_server.load(asset_path.to_owned()))
    }

    fn is_ready(asset: &Self::Asset, asset_server: &AssetServer) -> bool {
        asset_server.is_loaded(asset)
    }
}
