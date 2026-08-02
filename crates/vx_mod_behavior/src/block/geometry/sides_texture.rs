use serde::Deserialize;

use crate::block::geometry::QuadTexture;

#[derive(Debug, Deserialize)]
pub enum SidesTexture {
    Uniform(QuadTexture),
}
