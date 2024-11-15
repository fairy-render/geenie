use relative_path::RelativePathBuf;

#[derive(Debug, thiserror::Error)]
pub enum GeenieError {
    #[error("io error: {0}")]
    Io(#[from] std::io::Error),
    #[error("duplicate path: {path}")]
    Duplicate { path: RelativePathBuf },
    #[error("file already exists: {path}")]
    Exists { path: RelativePathBuf },
    #[error("command failed: {error}")]
    Process { error: String },
}

impl GeenieError {
    pub fn is_io(&self) -> bool {
        matches!(self, Self::Io(_))
    }

    pub fn duplicate(path: RelativePathBuf) -> GeenieError {
        GeenieError::Duplicate { path }
    }

    pub fn exists(path: RelativePathBuf) -> GeenieError {
        GeenieError::Exists { path }
    }

    pub fn command(error: String) -> GeenieError {
        GeenieError::Process { error }
    }
}
