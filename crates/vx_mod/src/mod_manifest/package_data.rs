use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct PackageData {
    name: String,
    version: String,
    manifest_format: String,
}

impl PackageData {
    #[must_use]
    pub const fn name(&self) -> &String {
        &self.name
    }

    #[must_use]
    pub const fn version(&self) -> &String {
        &self.version
    }

    #[must_use]
    pub const fn manifest_format(&self) -> &String {
        &self.manifest_format
    }
}
