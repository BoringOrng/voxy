use std::convert::Infallible;

use bevy::prelude::*;

pub struct TextureLoader;

impl super::Loader for TextureLoader {
    type Asset = Handle<Image>;
    type Error = Infallible;

    const DIR: &'static str = "texture";

    fn extensions() -> &'static [&'static str] {
        &["png", "jpg", "jpeg"]
    }

    fn try_load(path: &str, asset_server: &AssetServer) -> Result<Self::Asset, Self::Error> {
        Ok(asset_server.load(path.to_owned()))
    }

    fn is_ready(asset: &Self::Asset, asset_server: &AssetServer) -> bool {
        asset_server.is_loaded(asset)
    }
}
