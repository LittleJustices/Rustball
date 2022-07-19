use std::{
    error::Error,
    fmt, num::ParseFloatError,
};

#[derive(Debug)]
pub enum MathError {
    PlaceholderError,           // placeholder
    TokenError(ParseFloatError),         // Fail to parse RPN token
    ExpressionError(String),    // Malformed expression
    SymbolError(String),        // Illegal symbols in expression
}

impl Error for MathError {}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::PlaceholderError     => write!(f, "Error handling TBA"),
            MathError::TokenError(why) => write!(f, "Some symbols must've gotten mixed up! Can you check your math? {}", why),
            MathError::ExpressionError(why) => write!(f, "Something's wrong with that expression ! ∑(✘Д✘๑ ) {}", why),
            MathError::SymbolError(why)     => write!(f, "`{}` Σ(・艸・○) What's this? I can't do math with that!", why)
        }
    }
}

impl From<ParseFloatError> for MathError {
    fn from(why: ParseFloatError) -> Self {
        MathError::TokenError(why)
    }
}
