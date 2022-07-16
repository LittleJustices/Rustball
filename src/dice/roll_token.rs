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
    Argument(Argument),
}

// Method for deciding which token to try to parse based on the initial character in a string goes here

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
            RollToken::Math(rpn_token) => Ok(rpn_token),
            RollToken::Pool(pool) => Ok(RpnToken::Number(pool.total().into())),
            RollToken::Argument(argument) => {
                match argument {
                    Argument::Array(_) => Err(MathError::PlaceholderError),
                    Argument::Single(number) => Ok(RpnToken::Number(number.into()))
                }
            },
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
        if let Some(reroll_string) = s.strip_prefix('r') {
            return Ok(RollToken::Reroll(reroll_string.parse()?));
        }
        if let Some(explode_string) = s.strip_prefix('e') {
            return Ok(RollToken::Explode(explode_string.parse()?));
        }

        Err(RollError::PlaceholderError)
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
    Once(Vec<u8>),
    Recursive(Vec<u8>),
    Additive(Vec<u8>),
}

impl FromStr for Explode {
    type Err = RollError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (mut explosion, argument) = {
            if let Some(r_arg) = s.strip_prefix('r') {
                (Explode::Recursive(Vec::<u8>::new()), r_arg)
            } else if let Some(a_arg) = s.strip_prefix('a') {
                (Explode::Additive(Vec::<u8>::new()), a_arg)
            } else {
                (Explode::Once(Vec::<u8>::new()), s)
            }
        };

        if let Some(multi_explosions) = argument.trim().strip_prefix('[').unwrap_or("").strip_suffix(']') {
            for number_str in multi_explosions.split_terminator(',') {
                match explosion {
                    Explode::Once(ref mut explode_numbers) => explode_numbers.push(number_str.trim().parse()?),
                    Explode::Recursive(ref mut explode_numbers) => explode_numbers.push(number_str.trim().parse()?),
                    Explode::Additive(ref mut explode_numbers) => explode_numbers.push(number_str.trim().parse()?),
                }
            }
        } else {
            match explosion {
                Explode::Once(ref mut explode_numbers) => explode_numbers.push(argument.trim().parse()?),
                Explode::Recursive(ref mut explode_numbers) => explode_numbers.push(argument.trim().parse()?),
                Explode::Additive(ref mut explode_numbers) => explode_numbers.push(argument.trim().parse()?),
            }
        }
        
        Ok(explosion)
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
        let mut reroll_numbers = Vec::<u8>::new();

        let (argument, recur) = match s.strip_prefix('r') {
            Some(r_arg) => (r_arg, true),
            None => (s, false)
        };

        if let Some(multi_rerolls) = argument.trim().strip_prefix('[').unwrap_or("").strip_suffix(']') {
            for number_str in multi_rerolls.split_terminator(',') {
                reroll_numbers.push(number_str.trim().parse()?);
            }
        } else {
            reroll_numbers.push(argument.trim().parse()?);
        }

        match recur {
            true => Ok(Reroll::Recursive(reroll_numbers)),
            false => Ok(Reroll::Once(reroll_numbers))
        }
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
        let strings_to_parse = ["1d20", "k3", "r1", "r[1, 2]", "rr3", "e10", "er[9, 10]", "ea[10]"];

        assert_eq!(RollToken::Keep(Keep::High(3)), strings_to_parse[1].parse().unwrap());
        assert_eq!(RollToken::Reroll(Reroll::Once([1].to_vec())), strings_to_parse[2].parse().unwrap());
        assert_eq!(RollToken::Reroll(Reroll::Once([1, 2].to_vec())), strings_to_parse[3].parse().unwrap());
        assert_eq!(RollToken::Reroll(Reroll::Recursive([3].to_vec())), strings_to_parse[4].parse().unwrap());
        assert_eq!(RollToken::Explode(Explode::Once([10].to_vec())), strings_to_parse[5].parse().unwrap());
        assert_eq!(RollToken::Explode(Explode::Recursive([9, 10].to_vec())), strings_to_parse[6].parse().unwrap());
        assert_eq!(RollToken::Explode(Explode::Additive([10].to_vec())), strings_to_parse[7].parse().unwrap());
    }
}
