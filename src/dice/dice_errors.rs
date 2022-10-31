use std::{
    error::Error,
    fmt,
    num,
};

#[derive(Debug)]
pub struct PlaceholderError;

#[derive(Debug)]
pub enum RollError {
    NotImplementedError,
    PlaceholderError,
    ParseError(num::ParseIntError),
    RetrieveError(String),
    TranslationError(String),
}

impl Error for RollError {}

impl fmt::Display for RollError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RollError::NotImplementedError => write!(f, "I'm sorry, I can't actually do that yet... (m´・ω・｀)m ｺﾞﾒﾝ…"),
            RollError::PlaceholderError => write!(f, "Error handling TBA"),
            RollError::ParseError(why) => write!(f, "☢ ((((；´ﾟДﾟ))) These dice are too spicy for me! ☢ ({})", why),
            RollError::RetrieveError(why) => write!(f, "Sorry, I lost your dice (m´・ω・｀)m ｺﾞﾒﾝ… ({})", why),
            RollError::TranslationError(why) => write!(f, "{} σ(・ω・,,｀)？ I don't know what that means in this context!", why),
        }
    }
}

impl From<num::ParseIntError> for RollError {
    fn from(error: num::ParseIntError) -> Self {
        RollError::ParseError(error)
    }
}
