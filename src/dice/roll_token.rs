use std::{
    convert::TryFrom, 
    str::FromStr
};

use crate::math::{
    rpn_token::RpnToken, 
    math_errors::MathError
};
use super::{
    dice_errors::RollError,
    dice_re::DICE_MATCH_RE,
    pool::Pool, 
};

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum RollToken {
    Math(RpnToken),
    Pool(Pool),
    Explode(Explode),
    Keep(Keep),
    Reroll(Reroll),
    Target(Target),
    Botch(Target),
}

impl From<RpnToken> for RollToken {
    fn from(rpn_token: RpnToken) -> Self {
        RollToken::Math(rpn_token)
    }
}

impl TryFrom<RollToken> for RpnToken {
    type Error = MathError;

    fn try_from(value: RollToken) -> Result<Self, Self::Error> {
        match value {
            RollToken::Math(rpn_token) => Ok(rpn_token),
            RollToken::Pool(pool) => Ok(RpnToken::Number(pool.total().into())),
            _ => Err(MathError::PlaceholderError)
        }
    }
}

impl FromStr for RollToken {
    type Err = RollError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // If it can be parsed into an rpn token, it's an rpn token
        if let Ok(rpn_token) = s.parse::<RpnToken>() {
            return Ok(rpn_token.into());
        }

        // Logic for parsing other strings into roll tokens goes here
        if s == "d" {
            return Ok(RollToken::Pool(Pool::new(0, 0, Keep::All)));
        }
        if let Some(captures) = DICE_MATCH_RE.captures(s) {
            let number = captures["number"].parse()?;
            let sides = captures["sides"].parse()?;
            return Ok(RollToken::Pool(Pool::new(number, sides, Keep::All)));
        }
        if let Some(keep_string) = s.strip_prefix('k') {
            if let Some(keep_number_str) = keep_string.strip_prefix('l') {
                return Ok(RollToken::Keep(Keep::Low(keep_number_str.parse()?)));
            } else if let Some(keep_number_str) = keep_string.strip_prefix('h') {
                return Ok(RollToken::Keep(Keep::High(keep_number_str.parse()?)));
            } else {
                return Ok(RollToken::Keep(Keep::High(keep_string.parse()?)));
            }
        }

        Err(RollError::PlaceholderError)
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Explode {
    Once(Vec<u8>),
    Recursive(Vec<u8>),
}

#[derive(Debug, PartialEq)]
pub enum Keep {
    Low(u8),
    High(u8),
    All,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Reroll {
    Once(Vec<u8>),
    Recursive(Vec<u8>),
    Additive(Vec<u8>),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Target {
    Single(u8),
    Complex(Vec<u8>)
}
