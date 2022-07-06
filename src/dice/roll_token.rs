use std::convert::TryFrom;

use crate::math::{
    rpn_token::RpnToken, 
    math_errors::MathError
};
use super::pool::Pool;

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

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Explode {
    Single(Vec<u8>),
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
