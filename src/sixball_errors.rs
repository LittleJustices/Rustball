use crate::{
    math::math_errors::MathError, 
    dice::dice_errors::RollError,
    scryfall::scryfall_errors::ScryfallError,
};
use std::{
    error::Error, fmt::{
        Display,
        Formatter,
        Result
    }
};

#[derive(Debug)]
pub enum SixballError {
    MathError(MathError),
    RollError(RollError),
    ScryfallError(ScryfallError),
}

impl Error for SixballError {}

impl Display for SixballError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            SixballError::MathError(why) => write!(f, "☢ Math error! ☢ ({})", why),
            SixballError::RollError(why) => write!(f, "☢ Roll error! ☢ ({})", why),
            SixballError::ScryfallError(why) => write!(f, "☢ Scryfall error! ☢ ({})", why),
        }
    }
}

impl From<MathError> for SixballError {
    fn from(why: MathError) -> Self {
        SixballError::MathError(why)
    }
}

impl From<RollError> for SixballError {
    fn from(why: RollError) -> Self {
        SixballError::RollError(why)
    }
}

impl From<ScryfallError> for SixballError {
    fn from(why: ScryfallError) -> Self {
        SixballError::ScryfallError(why)
    }
}
