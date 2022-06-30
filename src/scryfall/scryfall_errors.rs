use reqwest;
use super::card::ErrorObject;
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
    ApiError(ErrorObject),
    ContentWarning,
    RequestError(reqwest::Error),
}

impl Error for ScryfallError {}

impl Display for ScryfallError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            ScryfallError::PlaceholderError => write!(f, "Error handling TBA"),
            ScryfallError::ApiError(why) => write!(f, "The Scryfall API gave me an error! (´∩｀。)ｸﾞｽﾝ\n{}", why),
            ScryfallError::ContentWarning => write!(f, "I'm not really comfortable showing you that card! If you need to look at it, please look it up on the actual scryfall site."),
            ScryfallError::RequestError(why) => write!(f, "Something went wrong with that request! ｺﾞﾒ─･ﾟ･(p´Д`q)･ﾟ･─ﾝ ({})", why)
        }
    }
}

impl From<reqwest::Error> for ScryfallError {
    fn from(why: reqwest::Error) -> Self {
        ScryfallError::RequestError(why)
    }
}

impl From<ErrorObject> for ScryfallError {
    fn from(why: ErrorObject) -> Self {
        ScryfallError::ApiError(why)
    }
}
