use chrono::prelude::*;
use std::fmt;
use crate::math::rpn_token::RpnToken;

use super::{
    dice_re::DICE_MATCH_RE,
    dice_errors::RollError,
    pool::Pool,
    roll_token::RollToken,
};

#[derive(Debug)]
pub struct Roll {
    command: String,
    comment: String,
    dicepools: Vec<Pool>,
    operations: Vec<RollToken>,
    result: f64,
    roller: String,
    timestamp: DateTime<Utc>,
}

impl Roll {
    pub fn new(command: &str, comment: &str) -> Result<Self, RollError> {
        let roller = "Placeholder name".to_owned();
        let timestamp = Utc::now();

        let (operations, result) = Roll::evaluate_string(command)?;

        let mut dicepools = Vec::new();
        for captures in DICE_MATCH_RE.captures_iter(command) {
            let number = captures["number"].parse::<u8>()?;
            let sides = captures["sides"].parse::<u8>()?;

            dicepools.push(Pool::new(number, sides));
        }
        Ok(Roll { command: command.to_string(), comment: comment.into(), dicepools, operations, result, roller, timestamp })
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
        for pool in self.dicepools.iter_mut() {
            pool.reroll_all();
        }
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
        &self.roller
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
                _ => continue
            }
        }

        write!(f, "{}", breakdown)
    }
}
