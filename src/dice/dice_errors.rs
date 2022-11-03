use std::{
    error::Error,
    fmt,
    num,
};
use crate::math::math_errors::MathError;

#[derive(Debug)]
pub struct PlaceholderError;

#[derive(Debug)]
pub enum RollError {
    ArgumentError,
    FBomb,
    MathError(MathError),
    MissingPoolError,
    NotANumberError,
    NotImplementedError,
    NotResolvedError,
    PlaceholderError,
    ParseError(num::ParseIntError),
    RetrieveError(String),
    SymbolError(String),
    TranslationError(String),
}

impl Error for RollError {}

impl fmt::Display for RollError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RollError::ArgumentError => write!(f, "ぇ━(*´･д･)━!!! I don't know what to do with this! (Failed to find an argument or wrong argument)"),
            RollError::FBomb => write!(f, "What the fuck"),
            RollError::MathError(why) => write!(f, "{}", why),
            RollError::MissingPoolError => write!(f, "Where'd the dice go!? !!!∑(ﾟﾛﾟ!(ﾟﾍﾟ?)??? I swear I was looking... (Failed to find a dicepool)"),
            RollError::NotANumberError => write!(f, "Hold up, that's not a number! ヾ(｡｀Д´｡)ﾉ彡☆ﾌﾞｰﾌﾞｰｯ!! (Tried to treat a non-numerical value as a number)"),
            RollError::NotImplementedError => write!(f, "I'm sorry, I can't actually do that yet... (m´・ω・｀)m ｺﾞﾒﾝ…"),
            RollError::NotResolvedError => write!(f, "Hooold up! (｡･_･｡)ﾉ ﾁｮｲﾏﾁ｡ Something's happening out of order here?? (Tried to use an operator before resolving it)"),
            RollError::PlaceholderError => write!(f, "Error handling TBA"),
            RollError::ParseError(why) => write!(f, "☢ ((((；´ﾟДﾟ))) These dice are too spicy for me! ☢ ({})", why),
            RollError::RetrieveError(why) => write!(f, "Sorry, I lost your dice (m´・ω・｀)m ｺﾞﾒﾝ… ({})", why),
            RollError::SymbolError(why) => write!(f, "{} Σ(・艸・○) What's this? I can't roll dice with that!", why),
            RollError::TranslationError(why) => write!(f, "{} σ(・ω・,,｀)？ I don't know what that means in this context!", why),
        }
    }
}

impl From<num::ParseIntError> for RollError {
    fn from(error: num::ParseIntError) -> Self {
        RollError::ParseError(error)
    }
}

impl From<MathError> for RollError {
    fn from(error: MathError) -> Self {
        RollError::MathError(error)
    }
}
