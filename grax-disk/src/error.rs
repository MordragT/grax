use thiserror::Error;

#[derive(Debug, Error)]
pub enum DiskGraphError {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
}

pub type DiskGraphResult<T> = Result<T, DiskGraphError>;
