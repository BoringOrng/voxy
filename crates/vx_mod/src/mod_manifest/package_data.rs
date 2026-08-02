use semver::Version;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PackageData {
    name: String,
    version: Version,
    manifest_format: Version,
}

impl PackageData {
    #[must_use]
    pub const fn name(&self) -> &String {
        &self.name
    }

    #[must_use]
    pub const fn version(&self) -> &Version {
        &self.version
    }

    #[must_use]
    pub const fn manifest_format(&self) -> &Version {
        &self.manifest_format
    }
}
