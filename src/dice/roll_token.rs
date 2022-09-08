use std::{
    convert::TryFrom, 
    str::FromStr, fmt
};

use crate::math::{
    rpn_token::RpnToken, 
    math_errors::MathError
};
use super::{
    dice_errors::RollError,
    dice_re::DICE_TOKEN_RE,
    pool::Pool,
};
pub use super::token_kinds::*;

#[derive(Clone, Debug, PartialEq)]
pub enum RollToken {
    Math(RpnToken),
    Dice(Dice),
    Argument(Argument),
    Operator(Operator),
    Conversion(Conversion),
}

impl RollToken {
    #[allow(dead_code)]
    pub fn precedence(&self) -> u8 {
        match self {
            RollToken::Math(rpn_token) => rpn_token.precedence(),
            _ => 255
        }
    }

    pub fn description(&self) -> String {
        match self {
            RollToken::Dice(dice) => dice.description(),
            RollToken::Operator(operator) => operator.description(),
            RollToken::Conversion(conversion) => conversion.description(),
            _ => "Placeholder description".into()
        }
    }

    pub fn verbose(&self) -> String {
        match self {
            RollToken::Dice(dice) => dice.verbose(),
            RollToken::Operator(operator) => operator.verbose(),
            RollToken::Conversion(conversion) => conversion.verbose(),
            _ => "Placeholder description".into()
        }
    }

    pub fn value(&self) -> Result<f64, RollError> {
        match self {
            RollToken::Math(rpn_token) => match rpn_token {
                RpnToken::Number(value) => Ok(*value),
                _ => Err(RollError::PlaceholderError),
            },
            RollToken::Argument(argument) => match argument {
                Argument::Array(_) => Err(RollError::NotImplementedError),
                Argument::Single(value) => {
                    let v = *value;
                    Ok(v.into())
                },
            },
            RollToken::Dice(dice) => dice.clone().value(),
            RollToken::Operator(operator) => operator.value(),
            RollToken::Conversion(conversion) => conversion.value(),
        }
    }

    pub fn argument(self) -> Result<Argument, RollError> {
        match self {
            RollToken::Argument(argument) => Ok(argument),
            RollToken::Dice(dice) => Ok(Argument::Single(dice.value()? as u8)),
            RollToken::Operator(operator) => Ok(Argument::Single(operator.value()? as u8)),
            _ => Err(RollError::PlaceholderError)
        }
    }

    pub fn pool(self) -> Result<Pool, RollError> {
        match self {
            RollToken::Dice(dice) => dice.pool(),
            RollToken::Operator(operator) => operator.pool(),
            RollToken::Conversion(conversion) => conversion.pool(),
            _ => Err(RollError::PlaceholderError)
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

    pub fn shunting_dice(infix_vector: &[RollToken]) -> Result<Vec<RollToken>, RollError> {
        let mut postfix_queue = vec![];
        let mut token_stack = vec![];

        for token in infix_vector.to_vec() {
            match &token {
                RollToken::Math(math_token) => match math_token {
                    RpnToken::LParen => token_stack.push(token),
                    RpnToken::RParen => {
                        while let Some(top_token) = token_stack.pop() {
                            if top_token == RollToken::Math(RpnToken::LParen) { break; };
                            postfix_queue.push(top_token);
                        }
                        if let Some(RollToken::Math(RpnToken::MathFn(_))) = token_stack.last() {
                            postfix_queue.push(token_stack.pop().ok_or(RollError::PlaceholderError)?);
                        }
                    },
                    RpnToken::Number(_) => postfix_queue.push(token),
                    RpnToken::MathFn(_) => token_stack.push(token),
                    RpnToken::Operator(right_operator) => {
                        while let Some(top_of_stack) = token_stack.last() {
                            match top_of_stack {
                                RollToken::Math(RpnToken::Operator(left_operator)) => {
                                    if (left_operator.precedence() > right_operator.precedence()) |
                                    ((left_operator.precedence() == right_operator.precedence()) && (right_operator.left_associative())) {
                                        postfix_queue.push(token_stack.pop().ok_or(RollError::PlaceholderError)?);
                                    } else {
                                        break;
                                    }
                                },
                                RollToken::Dice(_) | RollToken::Operator(_) => postfix_queue.push(token_stack.pop().ok_or(RollError::PlaceholderError)?),
                                _ => break
                            }
                        }
                        token_stack.push(token);
                    },
                },
                RollToken::Argument(_) => postfix_queue.push(token),
                RollToken::Dice(_) => {
                    while let Some(RollToken::Dice(_)) = token_stack.last() {
                        postfix_queue.push(token_stack.pop().ok_or(RollError::PlaceholderError)?);
                    }
                    token_stack.push(token);
                },
                RollToken::Operator(_) | RollToken::Conversion(_) => {
                    while let Some(top_of_stack) = token_stack.last() {
                        match top_of_stack {
                            RollToken::Dice(_) | RollToken::Operator(_) | RollToken::Conversion(_) => postfix_queue.push(token_stack.pop().ok_or(RollError::PlaceholderError)?),
                            _ => break
                        }
                    }
                    token_stack.push(token);
                },
            }
        }

        while let Some(token) = token_stack.pop() {
            match token {
                RollToken::Math(RpnToken::LParen) | RollToken::Math(RpnToken::RParen) => return Err(RollError::PlaceholderError),
                other => postfix_queue.push(other)
            }
        }

        Ok(postfix_queue)
    }
}

impl From<RpnToken> for RollToken {
    fn from(rpn_token: RpnToken) -> Self {
        match rpn_token {
            RpnToken::Number(number) => {
                let number_as_argument = number as u8;
                if number == number_as_argument as f64 {
                    RollToken::Argument(Argument::Single(number_as_argument))
                } else {
                    RollToken::Math(rpn_token)
                }
            },
            _ => RollToken::Math(rpn_token),
        }
    }
}

impl TryFrom<RollToken> for RpnToken {
    type Error = MathError;

    fn try_from(value: RollToken) -> Result<Self, Self::Error> {
        match value {
            RollToken::Math(rpn_token)      => Ok(rpn_token),
            RollToken::Dice(dice)   => Ok(RpnToken::Number(dice.value().or(Err(MathError::PlaceholderError))?)),
            RollToken::Operator(operator)   => Ok(RpnToken::Number(operator.value().or(Err(MathError::PlaceholderError))?)),
            RollToken::Conversion(conversion)   => Ok(RpnToken::Number(conversion.value().or(Err(MathError::PlaceholderError))?)),
            RollToken::Argument(argument)   => {
                match argument {
                    Argument::Array(_)                => Err(MathError::PlaceholderError),
                    Argument::Single(number)      => Ok(RpnToken::Number(number.into()))
                }
            },
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
        } else if let Ok(operator) = s.parse() {          // Attempt to parse into operator
            Ok(RollToken::Operator(operator))
        } else if let Ok(conversion) = s.parse() {          // Attempt to parse into operator
            Ok(RollToken::Conversion(conversion))
        } else {                                                  // If all these fail, error out
            Err(RollError::PlaceholderError)
        }
    }
}

impl fmt::Display for RollToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RollToken::Dice(dice) => write!(f, "{}", dice),
            RollToken::Operator(operator) => write!(f, "{}", operator),
            _ => write!(f, "")
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::math::rpn_token;

    use super::*;

    #[test]
    fn test_from_str() {
        let expression = "2d20kh1";
        let token_vector = vec![
            RollToken::Argument(Argument::Single(2)),
            RollToken::Dice(Dice{ pool: None }),
            RollToken::Argument(Argument::Single(20)),
            RollToken::Operator(Operator::Keep(Keep::High { arg: None, res: None })),
            RollToken::Argument(Argument::Single(1)),
        ];

        assert_eq!(RollToken::tokenize_expression(expression).unwrap(), token_vector);
    }

    #[test]
    fn test_shunting_dice() {
        let expressions = vec![
            "(2d6r[1,2]*1.5)/2",
            "1d2+3d4+5d6",
            "(2+2)d10+4d(5+5)",
            "4d6r1k3",
            "1d2d3d4",
        ];
        let token_vector_0 = RollToken::tokenize_expression(expressions[0]).unwrap();
        let token_vector_1 = RollToken::tokenize_expression(expressions[1]).unwrap();
        let token_vector_2 = RollToken::tokenize_expression(expressions[2]).unwrap();
        let token_vector_3 = RollToken::tokenize_expression(expressions[3]).unwrap();
        let token_vector_4 = RollToken::tokenize_expression(expressions[4]).unwrap();

        let postfix_0 = vec![
            RollToken::Argument(Argument::Single(2)),
            RollToken::Argument(Argument::Single(6)),
            RollToken::Dice(Dice{ pool: None }),
            RollToken::Argument(Argument::Array(vec![1, 2])),
            RollToken::Operator(Operator::Reroll(Reroll::Once { arg: None , res: None, rerolls: None })),
            RollToken::Math(RpnToken::Number(1.5)),
            RollToken::Math(RpnToken::Operator(rpn_token::Operator::Mul)),
            RollToken::Argument(Argument::Single(2)),
            RollToken::Math(RpnToken::Operator(rpn_token::Operator::Div)),
        ];
        let postfix_1 = vec![
            RollToken::Argument(Argument::Single(1)),
            RollToken::Argument(Argument::Single(2)),
            RollToken::Dice(Dice{ pool: None }),
            RollToken::Argument(Argument::Single(3)),
            RollToken::Argument(Argument::Single(4)),
            RollToken::Dice(Dice{ pool: None }),
            RollToken::Math(RpnToken::Operator(rpn_token::Operator::Add)),
            RollToken::Argument(Argument::Single(5)),
            RollToken::Argument(Argument::Single(6)),
            RollToken::Dice(Dice{ pool: None }),
            RollToken::Math(RpnToken::Operator(rpn_token::Operator::Add)),
        ];
        let postfix_2 = vec![
            RollToken::Argument(Argument::Single(2)),
            RollToken::Argument(Argument::Single(2)),
            RollToken::Math(RpnToken::Operator(rpn_token::Operator::Add)),
            RollToken::Argument(Argument::Single(10)),
            RollToken::Dice(Dice{ pool: None }),
            RollToken::Argument(Argument::Single(4)),
            RollToken::Argument(Argument::Single(5)),
            RollToken::Argument(Argument::Single(5)),
            RollToken::Math(RpnToken::Operator(rpn_token::Operator::Add)),
            RollToken::Dice(Dice{ pool: None }),
            RollToken::Math(RpnToken::Operator(rpn_token::Operator::Add)),
        ];
        let postfix_3 = vec![
            RollToken::Argument(Argument::Single(4)),
            RollToken::Argument(Argument::Single(6)),
            RollToken::Dice(Dice{ pool: None }),
            RollToken::Argument(Argument::Single(1)),
            RollToken::Operator(Operator::Reroll(Reroll::Once { arg: None , res: None, rerolls: None })),
            RollToken::Argument(Argument::Single(3)),
            RollToken::Operator(Operator::Keep(Keep::High { arg: None, res: None })),
        ];
        let postfix_4 = vec![
            RollToken::Argument(Argument::Single(1)),
            RollToken::Argument(Argument::Single(2)),
            RollToken::Dice(Dice{ pool: None }),
            RollToken::Argument(Argument::Single(3)),
            RollToken::Dice(Dice{ pool: None }),
            RollToken::Argument(Argument::Single(4)),
            RollToken::Dice(Dice{ pool: None }),
        ];

        assert_eq!(RollToken::shunting_dice(&token_vector_0).unwrap(), postfix_0);
        assert_eq!(RollToken::shunting_dice(&token_vector_1).unwrap(), postfix_1);
        assert_eq!(RollToken::shunting_dice(&token_vector_2).unwrap(), postfix_2);
        assert_eq!(RollToken::shunting_dice(&token_vector_3).unwrap(), postfix_3);
        assert_eq!(RollToken::shunting_dice(&token_vector_4).unwrap(), postfix_4);
    }

    #[test]
    fn test_shunting_math() {
        let expression = "3+4*2/(1-5)^2^3";
        let token_vector = RollToken::tokenize_expression(expression).unwrap();

        let infix = vec![
            RollToken::Argument(Argument::Single(3)),
            RollToken::Math(RpnToken::Operator(rpn_token::Operator::Add)),
            RollToken::Argument(Argument::Single(4)),
            RollToken::Math(RpnToken::Operator(rpn_token::Operator::Mul)),
            RollToken::Argument(Argument::Single(2)),
            RollToken::Math(RpnToken::Operator(rpn_token::Operator::Div)),
            RollToken::Math(RpnToken::LParen),
            RollToken::Argument(Argument::Single(1)),
            RollToken::Math(RpnToken::Operator(rpn_token::Operator::Sub)),
            RollToken::Argument(Argument::Single(5)),
            RollToken::Math(RpnToken::RParen),
            RollToken::Math(RpnToken::Operator(rpn_token::Operator::Pow)),
            RollToken::Argument(Argument::Single(2)),
            RollToken::Math(RpnToken::Operator(rpn_token::Operator::Pow)),
            RollToken::Argument(Argument::Single(3)),
        ];

        let postfix = vec![
            RollToken::Argument(Argument::Single(3)),
            RollToken::Argument(Argument::Single(4)),
            RollToken::Argument(Argument::Single(2)),
            RollToken::Math(RpnToken::Operator(rpn_token::Operator::Mul)),
            RollToken::Argument(Argument::Single(1)),
            RollToken::Argument(Argument::Single(5)),
            RollToken::Math(RpnToken::Operator(rpn_token::Operator::Sub)),
            RollToken::Argument(Argument::Single(2)),
            RollToken::Argument(Argument::Single(3)),
            RollToken::Math(RpnToken::Operator(rpn_token::Operator::Pow)),
            RollToken::Math(RpnToken::Operator(rpn_token::Operator::Pow)),
            RollToken::Math(RpnToken::Operator(rpn_token::Operator::Div)),
            RollToken::Math(RpnToken::Operator(rpn_token::Operator::Add)),
        ];

        assert_eq!(token_vector, infix);
        assert_eq!(RollToken::shunting_dice(&token_vector).unwrap(), postfix);
    }
}
