use serde::Deserialize;

mod quad_texture;
mod sides_texture;

pub use quad_texture::QuadTexture;
pub use sides_texture::SidesTexture;

#[derive(Debug, Deserialize, Clone)]
pub enum BlockGeometry {
    Cube {
        top: QuadTexture,
        bottom: QuadTexture,
        sides: SidesTexture,
    },
}
