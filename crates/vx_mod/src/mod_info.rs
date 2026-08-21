use std::{
    fs,
    path::{Path, PathBuf},
};

use crate::{ModManifest, error::Error};

#[derive(Debug)]
pub struct ModInfo {
    id: String,
    root: PathBuf,
}

impl ModInfo {
    pub const MODS_DIR: &str = "mods/";
    pub const MODS_SOURCE: &str = "mods://";

    #[must_use]
    pub const fn new(id: String, root: PathBuf) -> Self {
        Self { id, root }
    }

    #[must_use]
    pub const fn id(&self) -> &String {
        &self.id
    }

    #[must_use]
    pub const fn root(&self) -> &PathBuf {
        &self.root
    }

    #[must_use]
    pub fn behavior_path(&self) -> PathBuf {
        PathBuf::from(Self::MODS_DIR)
            .join(&self.root)
            .join("behavior/")
    }

    #[must_use]
    pub fn asset_path(&self) -> PathBuf {
        PathBuf::from(Self::MODS_SOURCE)
            .join(&self.root)
            .join("resource/")
    }

    #[must_use]
    pub fn real_asset_path(&self) -> PathBuf {
        PathBuf::from(Self::MODS_DIR)
            .join(&self.root)
            .join("resource/")
    }

    pub(crate) fn load(root: &Path) -> Result<Self, Error> {
        if !root.join("mod.toml").exists() {
            return Err(Error::MissingManifest);
        }

        let manifest = fs::read_to_string(root.join("mod.toml"))?;
        let parsed: ModManifest = toml::from_str(&manifest)?;

        Ok(Self {
            id: parsed.package().name().to_owned(),
            root: root
                .strip_prefix(Self::MODS_DIR)
                .expect("mods should live in the mods folder")
                .to_owned(),
        })
    }
}
