use std::{
    error::Error,
    fmt,
};

#[derive(Debug)]
pub enum MathError {
    PlaceholderError,
}

impl Error for MathError {}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::PlaceholderError => write!(f, "Error handling TBA"),
        }
    }
}