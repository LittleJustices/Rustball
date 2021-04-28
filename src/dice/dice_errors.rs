use std::{
    error::Error,
    fmt,
    num,
};

#[derive(Debug)]
pub struct PlaceholderError;

#[derive(Debug)]
pub enum RollParseError {
    InputError(String),
    ParseError(num::ParseIntError),
}

impl Error for RollParseError {}

impl fmt::Display for RollParseError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RollParseError::InputError(why) => write!(f, "{}", why),
            RollParseError::ParseError(why) => write!(f, "{}", why),
        }
    }
}

impl From<num::ParseIntError> for RollParseError {
    fn from(error: num::ParseIntError) -> Self {
        RollParseError::ParseError(error)
    }
}

