use serde::Deserialize;

mod dependency_data;
mod package_data;

pub use dependency_data::DependencyData;
pub use package_data::PackageData;

#[derive(Debug, Deserialize)]
pub struct ModManifest {
    package: PackageData,
    dependencies: DependencyData,
}

impl ModManifest {
    #[must_use]
    pub const fn package(&self) -> &PackageData {
        &self.package
    }

    #[must_use]
    pub const fn dependencies(&self) -> &DependencyData {
        &self.dependencies
    }
}
