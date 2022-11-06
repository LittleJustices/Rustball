use chrono::prelude::*;
use std::fmt;

use super::{
    dice_errors::RollError,
    roll_stack::RollStack,
    roll_token::RollToken,
    roll_value::RollValue,
};

#[derive(Debug)]
pub struct Roll {
    command: String,
    comment: String,
    operations: RollStack,
    result: RollValue,
    owner: String,
    timestamp: DateTime<Utc>,
}

impl Roll {
    pub fn new(expression: &str, comment: &str, roller: &str) -> Result<Self, RollError> {
        let command = expression.to_string();
        let owner = roller.to_string();
        let comment = comment.to_string();
        let timestamp = Utc::now();

        let operations = RollStack::evaluate_string(expression)?;
        let result = operations.final_result.value()?;

        Ok(Roll { command, comment, operations, result, owner, timestamp })
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
        &self.operations.operations
    }

    pub fn result(&self) -> &RollValue {
        &self.result
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
        let operations = self.operations();

        if operations.len() == 0 { return write!(f, "No dice rolled"); }

        let mut breakdown = format!("{}", operations[0]);
        for i in 1..operations.len() {
            match &operations[i] {
                RollToken::Dice(dice) => breakdown = format!("{}; {}", breakdown, dice),
                RollToken::Operator(operator) => breakdown = format!("{}, {}", breakdown, operator),
                RollToken::Conversion(conversion) => breakdown = format!("{}, {}", breakdown, conversion),
                _ => continue
            }
        }

        write!(f, "{}", breakdown)
    }
}
