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
    dice_re::DICE_TOKEN_RE,
};
pub use super::token_kinds::*;

#[derive(Clone, Debug, PartialEq)]
pub enum RollToken {
    Math(RpnToken),
    Dice(Dice),
    Explode(Explode),
    Keep(Keep),
    Reroll(Reroll),
    Target(Target),
    Argument(Argument),
}

impl RollToken {
    #[allow(dead_code)]
    pub fn precedence(&self) -> u8 {
        match self {
            RollToken::Math(rpn_token) => rpn_token.precedence(),
            _ => 255
        }
    }

    pub fn tokenize_expression(infix_expression: &str) -> Result<Vec<RollToken>, RollError> {
        let whitespace_cleaned = infix_expression.replace(" ", "");
        let infix_processed = DICE_TOKEN_RE.replace_all(&whitespace_cleaned, " $token ");

        let mut infix_vector = vec![];
        for symbol in infix_processed.split_whitespace() {
            infix_vector.push(symbol.parse()?);
        }

        Ok(infix_vector)
    }
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
            RollToken::Dice(Dice{ pool: Some(pool) })   => Ok(RpnToken::Number(pool.total().into())),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let expression = "2d20kh1";
        let token_vector = vec![
            RollToken::Argument(Argument::Single(2)),
            RollToken::Dice(Dice{ pool: None }),
            RollToken::Argument(Argument::Single(20)),
            RollToken::Keep(Keep::High(None)),
            RollToken::Argument(Argument::Single(1)),
        ];

        assert_eq!(RollToken::tokenize_expression(expression).unwrap(), token_vector);
    }
}
