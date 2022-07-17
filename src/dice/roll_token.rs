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
    pool::Pool, 
};

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum RollToken {
    Math(RpnToken),
    Dice(Dice),
    Explode(Explode),
    Keep(Keep),
    Reroll(Reroll),
    Target(Target),
    Botch(Target),
    Argument(Argument),
}

impl From<RpnToken> for RollToken {
    fn from(rpn_token: RpnToken) -> Self {
        if let RpnToken::Number(number) = rpn_token {
            RollToken::Argument(Argument::Single(number as u8))
        } else {
            RollToken::Math(rpn_token)
        }
    }
}

impl TryFrom<RollToken> for RpnToken {
    type Error = MathError;

    fn try_from(value: RollToken) -> Result<Self, Self::Error> {
        match value {
            RollToken::Math(rpn_token)      => Ok(rpn_token),
            RollToken::Dice(Dice(Some(pool)))   => Ok(RpnToken::Number(pool.total().into())),
            RollToken::Argument(argument)   => {
                match argument {
                    Argument::Array(_)                => Err(MathError::PlaceholderError),
                    Argument::Single(number)      => Ok(RpnToken::Number(number.into()))
                }
            },
            _ => Err(MathError::PlaceholderError)
        }
    }
}

impl FromStr for RollToken {
    type Err = RollError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(argument) = s.parse() {               // Attempt to parse into argument token
            Ok(RollToken::Argument(argument))
        } else if let Ok(rpn_token) = s.parse() {       // Attempt to parse into math token
            Ok(RollToken::Math(rpn_token))
        } else if let Ok(dice) = s.parse() {                // Attempt to parse into pool token
            Ok(RollToken::Dice(dice))
        } else if let Ok(explode) = s.parse() {          // Attempt to parse into explode token
            Ok(RollToken::Explode(explode))
        } else if let Ok(keep) = s.parse() {                // Attempt to parse into keep token
            Ok(RollToken::Keep(keep))
        } else if let Ok(reroll) = s.parse() {            // Attempt to parse into reroll token
            Ok(RollToken::Reroll(reroll))
        } else if let Ok(target) = s.parse() {            // Attempt to parse into target token
            Ok(RollToken::Target(target))
        } else {                                                  // If all these fail, error out
            Err(RollError::PlaceholderError)
        }
    }
}

#[derive(Debug, PartialEq)]
pub enum Argument {
    Single(u8),
    Array(Vec<u8>),
}

impl FromStr for Argument {
    type Err = RollError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(array_string) = s.trim().strip_prefix('[').unwrap_or("").strip_suffix(']') {
            let mut args_array = Vec::<u8>::new();
            for number_str in array_string.split_terminator(',') {
                args_array.push(number_str.trim().parse()?);
            }
            Ok(Argument::Array(args_array))
        } else {
            Ok(Argument::Single(s.parse()?))
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Explode {
    Once(Option<Argument>),
    Recursive(Option<Argument>),
    Additive(Option<Argument>),
}

impl FromStr for Explode {
    type Err = RollError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(mode) = s.trim().strip_prefix('r') {
            match mode {
                "" | "o"    => Ok(Explode::Once(None)),
                "r"         => Ok(Explode::Recursive(None)),
                "a"         => Ok(Explode::Additive(None)),
                _           => Err(RollError::PlaceholderError)
            }
        } else {
            Err(RollError::PlaceholderError)
        }
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
        if let Some(mode) = s.trim().strip_prefix('k') {
            match mode {
                "" | "h"    => Ok(Keep::High(0)),
                "l"         => Ok(Keep::Low(0)),
                _           => Err(RollError::PlaceholderError)
            }
        } else {
            Err(RollError::PlaceholderError)
        }
    }
}

#[derive(Debug, PartialEq)]
pub struct Dice(Option<Pool>);

impl FromStr for Dice {
    type Err = RollError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "d" {                               // If just the dice operator, return an uninitialized pool
            Ok(Dice(None))
        } else if let Ok(pool) = s.parse() {  // If it can be parsed into a pool, return that pool
            Ok(Dice(Some(pool)))
        } else {                                    // Otherwise error
            Err(RollError::PlaceholderError)
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Reroll {
    Once(Option<Argument>),
    Recursive(Option<Argument>),
}

impl FromStr for Reroll {
    type Err = RollError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(mode) = s.trim().strip_prefix('r') {
            match mode {
                "" | "o"    => Ok(Reroll::Once(None)),
                "r"         => Ok(Reroll::Recursive(None)),
                _           => Err(RollError::PlaceholderError)
            }
        } else {
            Err(RollError::PlaceholderError)
        }
    }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Target {
    Success(Option<Argument>),
    Botch(Option<Argument>)
}

impl FromStr for Target {
    type Err = RollError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "t" => Ok(Target::Success(None)),
            "b" => Ok(Target::Botch(None)),
            _   => Err(RollError::PlaceholderError)
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn test_from_str() {
        todo!()
        // let strings_to_parse = ["1d20", "k3", "r1", "r[1, 2]", "rr3", "e10", "er[9, 10]", "ea[10]"];

        // assert_eq!(RollToken::Keep(Keep::High(3)), strings_to_parse[1].parse().unwrap());
        // assert_eq!(RollToken::Reroll(Reroll::Once([1].to_vec())), strings_to_parse[2].parse().unwrap());
        // assert_eq!(RollToken::Reroll(Reroll::Once([1, 2].to_vec())), strings_to_parse[3].parse().unwrap());
        // assert_eq!(RollToken::Reroll(Reroll::Recursive([3].to_vec())), strings_to_parse[4].parse().unwrap());
        // assert_eq!(RollToken::Explode(Explode::Once([10].to_vec())), strings_to_parse[5].parse().unwrap());
        // assert_eq!(RollToken::Explode(Explode::Recursive([9, 10].to_vec())), strings_to_parse[6].parse().unwrap());
        // assert_eq!(RollToken::Explode(Explode::Additive([10].to_vec())), strings_to_parse[7].parse().unwrap());
    }
}
