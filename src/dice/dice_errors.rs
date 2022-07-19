use std::{
    error::Error,
    fmt,
    num,
};

#[derive(Debug)]
pub struct PlaceholderError;

#[derive(Debug)]
pub enum RollError {
    PlaceholderError,
    ParseError(num::ParseIntError),
    RetrieveError(String),
}

impl Error for RollError {}

impl fmt::Display for RollError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RollError::PlaceholderError => write!(f, "Error handling TBA"),
            RollError::ParseError(why) => write!(f, "☢ ((((；´ﾟДﾟ))) These dice are too spicy for me! ☢ ({})", why),
            RollError::RetrieveError(why) => write!(f, "Sorry, I lost your dice (m´・ω・｀)m ｺﾞﾒﾝ… ({})", why),
        }
    }
}

impl From<num::ParseIntError> for RollError {
    fn from(error: num::ParseIntError) -> Self {
        RollError::ParseError(error)
    }
}
