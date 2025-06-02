use thiserror::Error;

/// Custom error type for the StalmarckSAT library
#[derive(Error, Debug)]
pub enum Error {
    /// I/O errors
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Parsing errors
    #[error("Parse error: {0}")]
    Parse(String),

    /// Invalid format errors
    #[error("Invalid format: {0}")]
    InvalidFormat(String),

    /// Variable out of bounds
    #[error("Variable out of bounds: {0}")]
    VariableOutOfBounds(i32),

    /// Solver errors
    #[error("Solver error: {0}")]
    Solver(String),

    /// Timeout occurred
    #[error("Solver timed out after {0} seconds")]
    Timeout(f64),
}
