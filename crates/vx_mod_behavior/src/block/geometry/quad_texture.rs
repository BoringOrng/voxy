use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub enum QuadTexture {
    Texture(String),
    ColorMapped { texture: String, color_map: String },
}

impl QuadTexture {
    #[must_use]
    pub const fn texture_id(&self) -> &String {
        match self {
            Self::Texture(texture) | Self::ColorMapped { texture, .. } => texture,
        }
    }
}
