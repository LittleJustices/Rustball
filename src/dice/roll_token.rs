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

// Method for deciding which token to try to parse based on the initial character in a string goes here

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
            return Ok(RollToken::Keep(keep_string.parse()?));
        }
        if let Some(target_string) = s.strip_prefix('t') {
            return Ok(RollToken::Target(target_string.parse()?));
        }

        Err(RollError::PlaceholderError)
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Explode {
    Once(Vec<u8>),
    Recursive(Vec<u8>),
    Additive(Vec<u8>),
}

impl FromStr for Explode {
    type Err = RollError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Keep {
    Low(u8),
    High(u8),
    All,
}

impl FromStr for Keep {
    type Err = RollError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(keep_high_number) = s.strip_prefix('h') {
            return Ok(Keep::High(keep_high_number.parse()?));
        } else if let Some(keep_low_number) = s.strip_prefix('l') {
            return Ok(Keep::Low(keep_low_number.parse()?));
        } else {
            return Ok(Keep::High(s.parse()?));
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Reroll {
    Once(Vec<u8>),
    Recursive(Vec<u8>),
}

impl FromStr for Reroll {
    type Err = RollError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Target {
    Single(u8),
    Complex(Vec<u8>)
}

impl FromStr for Target {
    type Err = RollError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(multi_targets) = s.trim().strip_prefix('[').unwrap_or("").strip_suffix(']') {
                let mut target_numbers = Vec::<u8>::new();
            for number_str in multi_targets.split_terminator(',') {
                target_numbers.push(number_str.trim().parse()?);
                }
                return Ok(Target::Complex(target_numbers));
            }
        return Ok(Target::Single(s.trim().parse()?));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let strings_to_parse = ["1d20", "k3"];

        assert_eq!(RollToken::Keep(Keep::High(3)), strings_to_parse[1].parse().unwrap());
    }
}
