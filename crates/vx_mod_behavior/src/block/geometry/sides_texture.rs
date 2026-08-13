use serde::Deserialize;

use crate::block::geometry::QuadTexture;

#[derive(Debug, Deserialize, Clone)]
pub enum SidesTexture {
    Uniform(QuadTexture),
}
