#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("`{0}` isn't inside the `assets` directory")]
    OutsideAssets(std::path::PathBuf),
}
