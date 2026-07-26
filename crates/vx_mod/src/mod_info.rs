use std::path::PathBuf;

#[derive(Debug)]
pub struct ModInfo {
    id: String,
    root: PathBuf,
}

impl ModInfo {
    #[must_use]
    pub const fn new(id: String, root: PathBuf) -> Self {
        Self { id, root }
    }

    #[must_use]
    pub const fn id(&self) -> &String {
        &self.id
    }

    #[must_use]
    pub const fn root(&self) -> &PathBuf {
        &self.root
    }
}
