//! Custom error, which the main LogParser uses
//!
//! Should indicate the type of error that occured, since there can be more than one error type.
//! Check the struct bellow.
#[derive(Debug)]
pub enum LogParserError {
    /// Indicates that the file could not be opened.
    /// Or, a reading from it returned an error.
    Io(std::io::Error),
    /// When reading the log lines, every parameter value in the query string should be an integer.
    /// Parsing as integer could fail.
    ParseIntError(std::num::ParseIntError),
    /// Could indicate some other error, like a query string missing in the url for example.
    Custom(String),
}

impl From<std::io::Error> for LogParserError {
    fn from(err: std::io::Error) -> LogParserError {
        LogParserError::Io(err)
    }
}

impl std::fmt::Display for LogParserError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            LogParserError::Io(e) => write!(f, "IO error: {}", e),
            LogParserError::ParseIntError(e) => write!(f, "Parse error: {}", e),
            LogParserError::Custom(msg) => write!(f, "Custom error: {}", msg),
        }
    }
}

impl std::error::Error for LogParserError {}
