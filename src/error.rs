use thiserror::Error;

pub type Result<T> = std::result::Result<T, AppError>;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    JsonError(#[from] serde_json::Error),

    #[error("CodeQL error: {0}")]
    CodeQLError(String),

    #[error("CSV parsing error: {0}")]
    CsvError(#[from] csv::Error),
}
