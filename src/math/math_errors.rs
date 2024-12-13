use std::{
    error::Error,
    fmt, num::ParseFloatError,
};

use super::rpn_token::RpnToken;

#[derive(Debug)]
pub enum MathError {
    PlaceholderError,           // placeholder
    ExpressionError(String),    // Malformed expression
    FnMismatchError,
    ImpossibleError,            // Error which shouldn't be possible
    MisplacedTokenError(RpnToken),
    OperatorMismatchError,
    ParensError,
    SymbolError(String),        // Illegal symbols in expression
    TokenError(ParseFloatError),         // Fail to parse RPN token
    TrailingTokensError,
}

impl Error for MathError {}

impl fmt::Display for MathError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            MathError::PlaceholderError     => write!(f, "Error handling TBA"),
            MathError::ExpressionError(why) => write!(f, "Something's wrong with that expression ! ∑(✘Д✘๑ ) {}", why),
            MathError::FnMismatchError => write!(f, "The functions and arguments don't match up! ∑(✘Д✘๑ ) Can you double check your expression?"),
            MathError::ImpossibleError => write!(f, "Congratulations, you managed to break me in a way the boss didn't think was possible! (유Д유〣) Please get their attention and describe exactly what you did."),
            MathError::MisplacedTokenError(what) => write!(f, "(╬ŎдŎ ) {:?} Whatever the hell this is, it doesn't belong here!", what),
            MathError::OperatorMismatchError => write!(f, "The operators and operands don't match up! ∑(✘Д✘๑ ) Can you double check your expression?"),
            MathError::ParensError => write!(f, "There's either too many or too few parentheses here! ∑(✘Д✘๑ ) Can you double check your expression?"),
            MathError::SymbolError(why)     => write!(f, "`{}` Σ(・艸・○) What's this? I can't do math with that!", why),
            MathError::TokenError(why) => write!(f, "Some symbols must've gotten mixed up! Can you check your math? {}", why),
            MathError::TrailingTokensError => write!(f, "I don't know how to finish resolving this! ∑(✘Д✘๑ ) Can you double check your expression?"),
        }
    }
}

impl From<ParseFloatError> for MathError {
    fn from(why: ParseFloatError) -> Self {
        MathError::TokenError(why)
    }
}
