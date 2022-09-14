use chrono::prelude::*;
use std::fmt;
use crate::math::rpn_token::RpnToken;

use super::{
    dice_errors::RollError,
    roll_token::RollToken,
};

#[derive(Debug)]
pub struct Roll {
    command: String,
    comment: String,
    operations: Vec<RollToken>,
    result: f64,
    owner: String,
    timestamp: DateTime<Utc>,
}

impl Roll {
    pub fn new(expression: &str, comment: &str, roller: &str) -> Result<Self, RollError> {
        let command = expression.to_string();
        let owner = roller.to_string();
        let timestamp = Utc::now();

        let (operations, result) = Roll::evaluate_string(expression)?;

        Ok(Roll { command, comment: comment.into(), operations, result, owner, timestamp })
    }

    fn evaluate_string(infix_expression: &str) -> Result<(Vec<RollToken>, f64), RollError> {
        let infix_tokens = RollToken::tokenize_expression(infix_expression)?;

        Roll::evaluate_tokens(&infix_tokens)
    }

    fn evaluate_tokens(infix_tokens: &[RollToken]) -> Result<(Vec<RollToken>, f64), RollError> {
        let postfix_tokens = RollToken::shunting_dice(infix_tokens)?;

        Roll::resolve_rpn(&postfix_tokens)
    }

    fn resolve_rpn(postfix_tokens: &[RollToken]) -> Result<(Vec<RollToken>, f64), RollError> {
        let tokens = postfix_tokens.to_vec();
        let mut stack = vec![];
        let mut operations = vec![];

        for token in tokens {
            match &token {
                RollToken::Math(rpn_token) => {
                    match rpn_token {
                        RpnToken::Number(_) => stack.push(token),
                        RpnToken::Operator(operator) => {
                            let right = stack.pop().ok_or(RollError::PlaceholderError)?;
                            let left = stack.pop().ok_or(RollError::PlaceholderError)?;
                            stack.push(RpnToken::Number(operator.apply(left.value()?, right.value()?)).into());
                        },
                        RpnToken::MathFn(math_fn) => {
                            let arg = stack.pop().ok_or(RollError::PlaceholderError)?;
                            stack.push(RpnToken::Number(math_fn.apply(arg.value()?)).into());
                        },
                        _ => return Err(RollError::PlaceholderError),
                    }
                },
                RollToken::Argument(_) => stack.push(token),
                RollToken::Dice(dice) => {
                    let right = stack.pop().ok_or(RollError::PlaceholderError)?;
                    let left = stack.pop().ok_or(RollError::PlaceholderError)?;
                    let dice_resolved = dice.apply(left.argument()?, right.argument()?)?;
                    operations.push(RollToken::Dice(dice_resolved.clone()));
                    stack.push(RollToken::Dice(dice_resolved));
                },
                RollToken::Operator(operator) => {
                    let right = stack.pop().ok_or(RollError::PlaceholderError)?;
                    let left = stack.pop().ok_or(RollError::PlaceholderError)?;
                    let operator_resolved = operator.apply(left.pool()?, right.argument()?)?;
                    operations.push(RollToken::Operator(operator_resolved.clone()));
                    stack.push(RollToken::Operator(operator_resolved));
                },
                RollToken::Conversion(conversion) => {
                    let right = stack.pop().ok_or(RollError::PlaceholderError)?;
                    let left = stack.pop().ok_or(RollError::PlaceholderError)?;
                    let conversion_resolved = conversion.apply(left, right.argument()?)?;
                    operations.push(RollToken::Conversion(conversion_resolved.clone()));
                    stack.push(RollToken::Conversion(conversion_resolved));
                },
            }
        }

        if stack.len() != 1 {
            Err(RollError::PlaceholderError)
        } else {
            Ok((operations, stack.pop().ok_or(RollError::PlaceholderError)?.value()?))
        }
    }

    #[allow(dead_code)]
    pub fn reroll_all(&mut self) {
        todo!();
    }

    pub fn command(&self) -> &str {
        &self.command
    }

    pub fn comment(&self) -> &str {
        &self.comment
    }

    pub fn operations(&self) -> &Vec<RollToken> {
        &self.operations
    }

    pub fn result(&self) -> f64 {
        self.result
    }

    pub fn roller(&self) -> &str {
        &self.owner
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
    }
}

impl fmt::Display for Roll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut breakdown = format!("{}", self.operations[0]);
        for i in 1..self.operations.len() {
            match &self.operations[i] {
                RollToken::Dice(dice) => breakdown = format!("{}; {}", breakdown, dice),
                RollToken::Operator(operator) => breakdown = format!("{}, {}", breakdown, operator),
                RollToken::Conversion(conversion) => breakdown = format!("{}, {}", breakdown, conversion),
                _ => continue
            }
        }

        write!(f, "{}", breakdown)
    }
}
