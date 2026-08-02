use serde::Deserialize;

pub mod geometry;
mod registry;

pub use geometry::BlockGeometry;
pub use registry::BlockRegistry;

#[derive(Debug, Deserialize)]
pub struct Block {
    id: String,
    geometry: BlockGeometry,
}

impl Block {
    #[must_use]
    pub const fn id(&self) -> &String {
        &self.id
    }

    #[must_use]
    pub const fn geometry(&self) -> &BlockGeometry {
        &self.geometry
    }
}
