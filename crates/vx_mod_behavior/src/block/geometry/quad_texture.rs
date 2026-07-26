use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum QuadTexture {
    Texture(String),
    ColorMapped { texture: String, color_map: String },
}
