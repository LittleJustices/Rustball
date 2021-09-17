use std::fmt;
use lazy_static::lazy_static;
use regex::Regex;
use std::str::FromStr;
use super::{
    dice_errors::RollParseError,
    pool::Pool,
};

lazy_static! {
    static ref DICE_MATCH_RE: Regex = Regex::new(r"\d+d\d+").expect("Failed to compile dice matching regex!");
    static ref DICE_SPLIT_RE: Regex = Regex::new(r"d").expect("Failed to compile dice splitting regex!");
}

#[derive(Debug)]
pub struct Roll {
    command: String,
    dicepool: Pool,
}

impl fmt::Display for Roll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} -> {}", self.command, self.dicepool)
    }
}

impl FromStr for Roll {
    type Err = RollParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !DICE_MATCH_RE.is_match(s) { return Err(RollParseError::InputError("Expected argument of the form XdY".to_owned())) }

        let args: Vec<&str> = DICE_SPLIT_RE.split(s).collect();
        let number = args[0].parse::<u8>()?;
        let sides = args[1].parse::<u8>()?;

        Ok(Roll { command: s.to_owned(), dicepool: Pool::new(number, sides) })
    }
}

mod tests {
    // use super::*;

    #[test]
    fn roll_from_string() {
        unimplemented!();
    }
}