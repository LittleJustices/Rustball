use std::str::FromStr;
use super::{
    dice_errors::RollError,
    pool::Pool,
};

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
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
    Low(Option<Argument>),
    High(Option<Argument>),
    All,
}

impl FromStr for Keep {
    type Err = RollError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(mode) = s.trim().strip_prefix('k') {
            match mode {
                "" | "h"    => Ok(Keep::High(None)),
                "l"         => Ok(Keep::Low(None)),
                _           => Err(RollError::PlaceholderError)
            }
        } else {
            Err(RollError::PlaceholderError)
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Dice(pub Option<Pool>);

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

#[derive(Clone, Debug, PartialEq)]
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

#[derive(Clone, Debug, PartialEq)]
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
