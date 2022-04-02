use std::{
    error::Error,
    fmt,
};

#[derive(Debug)]
pub enum MathParseError {
    PlaceholderError,
}

impl Error for MathParseError {}

impl fmt::Display for MathParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathParseError::PlaceholderError => write!(f, "Error handling TBA"),
        }
    }
}