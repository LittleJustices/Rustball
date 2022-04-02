use std::{
    error::Error,
    fmt,
};

#[derive(Debug)]
pub enum MathError {
    PlaceholderError,           // placeholder
    ExpressionError(String),    // Malformed expression
    SymbolError(String),        // Illegal symbols in expression
}

impl Error for MathError {}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::PlaceholderError     => write!(f, "Error handling TBA"),
            MathError::ExpressionError(why) => write!(f, "Something's wrong with that expression ! ∑(✘Д✘๑ ) {}", why),
            MathError::SymbolError(why)     => write!(f, "`{}` Σ(・艸・○) What's this? I can't do math with that!", why)
        }
    }
}