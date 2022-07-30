use std::{str::FromStr, fmt};
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

impl fmt::Display for Argument {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Argument::Array(array) => write!(f, "{:?}", array),
            Argument::Single(single) => write!(f, "{}", single),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Explode(Explode),
    Keep(Keep),
    Reroll(Reroll),
    Target(Target),
}

impl Operator {
    pub fn apply(&self, pool: Pool, argument: Argument) -> Result<Self, RollError> {
        match self {
            Operator::Keep(keep) => Ok(Operator::Keep(keep.apply(pool, argument)?)),
            _ => Err(RollError::NotImplementedError)
        }
    }

    pub fn pool(self) -> Result<Pool, RollError> {
        match self {
            Operator::Keep(keep) => keep.pool(),
            _ => Err(RollError::NotImplementedError)
        }
    }

    pub fn value(&self) -> Result<f64, RollError> {
        match self {
            Operator::Keep(keep) => keep.value(),
            _ => Err(RollError::NotImplementedError)
        }
    }

    pub fn verbose(&self) -> String {
        match self {
            Operator::Keep(keep) => keep.verbose(),
            _ => "You shouldn't be seeing this! Please let the boss know something's wrong!".into()
        }
    }
}

impl FromStr for Operator {
    type Err = RollError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(explode) = s.parse() {          // Attempt to parse into explode token
            Ok(Operator::Explode(explode))
        } else if let Ok(keep) = s.parse() {                // Attempt to parse into keep token
            Ok(Operator::Keep(keep))
        } else if let Ok(reroll) = s.parse() {            // Attempt to parse into reroll token
            Ok(Operator::Reroll(reroll))
        } else if let Ok(target) = s.parse() {            // Attempt to parse into target token
            Ok(Operator::Target(target))
        } else {                                                  // If all these fail, error out
            Err(RollError::PlaceholderError)
        }
    }
}

impl fmt::Display for Operator {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operator::Keep(keep) => write!(f, "{}", keep),
            _ => write!(f, ""),
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
        if let Some(mode) = s.trim().strip_prefix('e') {
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
    Low{arg: Option<Argument>, res: Option<Pool>},
    High{arg: Option<Argument>, res: Option<Pool>},
}

impl Keep {
    pub fn apply(&self, pool: Pool, argument: Argument) -> Result<Self, RollError> {
        let arg = Some(argument.clone());

        match self {
            Keep::High { arg: _, res: _ } => {
                match argument {
                    Argument::Array(_) => Err(RollError::PlaceholderError),
                    Argument::Single(keep_amount) => {
                        let res = Some(pool.keep_highest(keep_amount));
                        Ok(Keep::High { arg, res })
                    }
                }
            },
            Keep::Low { arg: _, res: _ } => {
                match argument {
                    Argument::Array(_) => Err(RollError::PlaceholderError),
                    Argument::Single(keep_amount) => {
                        let res = Some(pool.keep_lowest(keep_amount));
                        Ok(Keep::Low { arg, res })
                    }
                }
            },
        }
    }

    pub fn pool(self) -> Result<Pool, RollError> {
        match self {
            Keep::High { arg: _, res: pool } => pool.ok_or(RollError::PlaceholderError),
            Keep::Low { arg: _, res: pool } => pool.ok_or(RollError::PlaceholderError),
        }
    }

    pub fn value(&self) -> Result<f64, RollError> {
        match self {
            Keep::High { arg: _, res: pool } => Ok(pool.as_ref().ok_or(RollError::PlaceholderError)?.total().into()),
            Keep::Low { arg: _, res: pool } => Ok(pool.as_ref().ok_or(RollError::PlaceholderError)?.total().into()),
        }
    }

    pub fn verbose(&self) -> String {
        match self {
            Keep::High { arg, res: _ } => format!("Keep highest {} di(c)e", arg.as_ref().unwrap_or(&Argument::Single(0))),
            Keep::Low { arg, res: _ } => format!("Keep lowest {} di(c)e", arg.as_ref().unwrap_or(&Argument::Single(0))),
        }
    }
}

impl FromStr for Keep {
    type Err = RollError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(mode) = s.trim().strip_prefix('k') {
            match mode {
                "" | "h"    => Ok(Keep::High { arg: None, res: None }),
                "l"         => Ok(Keep::Low { arg: None, res: None }),
                _           => Err(RollError::PlaceholderError)
            }
        } else {
            Err(RollError::PlaceholderError)
        }
    }
}

impl fmt::Display for Keep {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Keep::High { arg, res } => write!(f, "kh {} -> {}", arg.as_ref().unwrap_or(&Argument::Single(0)), res.as_ref().unwrap_or(&Pool::new(0, 0))),
            Keep::Low { arg, res } => write!(f, "kl {} -> {}", arg.as_ref().unwrap_or(&Argument::Single(0)), res.as_ref().unwrap_or(&Pool::new(0, 0))),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct Dice{pub pool: Option<Pool>}

impl Dice {
    pub fn apply(&self, left: Argument, right: Argument) -> Result<Self, RollError> {
        let number = match left {
            Argument::Single(no) => no,
            Argument::Array(_) => return Err(RollError::PlaceholderError)
        };
        let sides = match right {
            Argument::Single(no) => no,
            Argument::Array(_) => return Err(RollError::PlaceholderError)
        };

        let pool = Some(Pool::new(number, sides));

        Ok(Dice{ pool })
    }

    pub fn verbose(&self) -> String {
        let pool = self.pool.as_ref().expect("Tried to print a dice operation that wasn't resolved yet!");
        format!("Rolled {}d{}", pool.number(), pool.sides())
    }
}

impl FromStr for Dice {
    type Err = RollError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "d" {                               // If just the dice operator, return an uninitialized pool
            Ok(Dice{ pool: None })
        } else if let Ok(pool) = s.parse() {  // If it can be parsed into a pool, return that pool
            Ok(Dice{ pool: Some(pool) })
        } else {                                    // Otherwise error
            Err(RollError::PlaceholderError)
        }
    }
}

impl fmt::Display for Dice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pool = self.pool.as_ref().expect("Tried to print a dice operation that wasn't resolved yet!");
        write!(f, "{}d{} -> {}", pool.number(), pool.sides(), pool)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Reroll {
    Better(Option<Argument>),
    Once(Option<Argument>),
    Recursive(Option<Argument>),
    Worse(Option<Argument>),
}

impl FromStr for Reroll {
    type Err = RollError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(mode) = s.trim().strip_prefix('r') {
            match mode {
                "" | "o"    => Ok(Reroll::Once(None)),
                "r"         => Ok(Reroll::Recursive(None)),
                "b"         => Ok(Reroll::Better(None)),
                "w"         => Ok(Reroll::Worse(None)),
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
