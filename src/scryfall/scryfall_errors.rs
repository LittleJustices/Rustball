use reqwest;
use std::{
    error::Error, fmt::{
        Display,
        Formatter,
        Result
    }
};

#[allow(dead_code)]
#[derive(Debug)]
pub enum ScryfallError {
    PlaceholderError,
    RequestError(reqwest::Error),
}

impl Error for ScryfallError {}

impl Display for ScryfallError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ScryfallError::PlaceholderError => write!(f, "Error handling TBA"),
            ScryfallError::RequestError(why) => write!(f, "Something went wrong with that request! ｺﾞﾒ─･ﾟ･(p´Д`q)･ﾟ･─ﾝ ({})", why)
        }
    }
}

impl From<reqwest::Error> for ScryfallError {
    fn from(why: reqwest::Error) -> Self {
        ScryfallError::RequestError(why)
    }
}
