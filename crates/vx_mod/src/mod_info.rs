use std::{fs, path::PathBuf};

use crate::{ModManifest, error::Error};

#[derive(Debug)]
pub struct ModInfo {
    id: String,
    root: PathBuf,
}

impl ModInfo {
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

    pub(crate) fn load(root: PathBuf) -> Result<Self, Error> {
        if !root.join("mod.toml").exists() {
            return Err(Error::MissingManifest);
        }

        let manifest = fs::read_to_string(root.join("mod.toml"))?;
        let parsed: ModManifest = toml::from_str(&manifest)?;

        Ok(Self {
            id: parsed.package().name().to_owned(),
            root,
        })
    }
}
