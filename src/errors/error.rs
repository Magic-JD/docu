use thiserror::Error;

#[derive(Debug, Error)]
pub enum DocuError {
    #[error("Database errors: {0}")]
    DatabaseSql(#[from] rusqlite::Error),

    #[error("Access errors: {0}")]
    Access(String),

    #[error("I/O errors: {0}")]
    Io(#[from] std::io::Error),
}
