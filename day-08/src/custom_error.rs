use thiserror::Error;

#[derive(Error, Debug)]
pub enum AocError {
    #[error(transparent)]
    IoError(#[from] std::io::Error),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Process error: {0}")]
    ProcessError(String),
}
