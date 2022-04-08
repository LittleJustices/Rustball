use std::{
    error::Error,
    fmt,
    num,
};

#[derive(Debug)]
pub struct PlaceholderError;

#[derive(Debug)]
pub enum RollError {
    InputError(String),
    ParseError(num::ParseIntError),
    RetrieveError(String),
}

impl Error for RollError {}

impl fmt::Display for RollError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            RollError::InputError(why) => write!(f, "{}", why),
            RollError::ParseError(why) => write!(f, "{}", why),
            RollError::RetrieveError(why) => write!(f, "{}", why),
        }
    }
}

impl From<num::ParseIntError> for RollError {
    fn from(error: num::ParseIntError) -> Self {
        RollError::ParseError(error)
    }
}
