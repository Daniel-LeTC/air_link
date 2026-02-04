use thiserror::Error;

#[derive(Error, Debug)]
pub enum AirLinkError {
    #[error("CLI error: {0}")]
    CliError(String),

    #[error("Internal core error: {0}")]
    CoreError(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("AI error: {0}")]
    Ort(#[from] ort::Error),

    #[error("Unknown error occurred")]
    Unknown,
}