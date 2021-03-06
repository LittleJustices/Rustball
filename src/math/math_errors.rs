use std::{
    error::Error,
    fmt, num::ParseFloatError,
};

#[derive(Debug)]
pub enum MathError {
    PlaceholderError,           // placeholder
    ExpressionError(String),    // Malformed expression
    ImpossibleError,            // Error which shouldn't be possible
    SymbolError(String),        // Illegal symbols in expression
    TokenError(ParseFloatError),         // Fail to parse RPN token
}

impl Error for MathError {}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::PlaceholderError     => write!(f, "Error handling TBA"),
            MathError::ExpressionError(why) => write!(f, "Something's wrong with that expression ! ∑(✘Д✘๑ ) {}", why),
            MathError::ImpossibleError => write!(f, "Congratulations, you managed to break me in a way the boss didn't think was possible! (유Д유〣) Please get their attention and describe exactly what you did."),
            MathError::SymbolError(why)     => write!(f, "`{}` Σ(・艸・○) What's this? I can't do math with that!", why),
            MathError::TokenError(why) => write!(f, "Some symbols must've gotten mixed up! Can you check your math? {}", why),
        }
    }
}

impl From<ParseFloatError> for MathError {
    fn from(why: ParseFloatError) -> Self {
        MathError::TokenError(why)
    }
}
