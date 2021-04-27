use std::num;

#[derive(Debug)]
pub struct PlaceholderError;

#[derive(Debug)]
pub enum RollParseError {
    InputError(String),
    ParseError(num::ParseIntError),
}

impl From<num::ParseIntError> for RollParseError {
    fn from(error: num::ParseIntError) -> Self {
        RollParseError::ParseError(error)
    }
}