#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("missing `mod.toml`")]
    MissingManifest,

    #[error("failed to read `mod.toml`: {0}")]
    Io(#[from] std::io::Error),

    #[error("failed to parse `mod.toml`: {0}")]
    Parse(#[from] toml::de::Error),
}
