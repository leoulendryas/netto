use thiserror::Error;

#[derive(Debug, Error)]
pub enum CodescanError {
    #[error("Failed to walk directory: {0}")]
    Walk(String),

    #[error("Failed to read file: {0}")]
    Io(#[from] std::io::Error),

    #[error("Git error: {0}")]
    Git(#[from] git2::Error),

    #[error("Not a git repository (run inside a git project for git stats)")]
    NotAGitRepo,

    #[error("Failed to serialize output: {0}")]
    Serialization(#[from] serde_json::Error),
}
