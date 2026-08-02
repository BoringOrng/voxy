#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to read block-data: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse block-data: {0}")]
    Parse(#[from] ron::de::SpannedError),
}
