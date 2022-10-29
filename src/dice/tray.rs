use std::collections::VecDeque;
use super::{
    dice_errors::RollError,
    roll::Roll,
    roll_value::RollValue,
};
use crate::sixball_errors::SixballError;

const CAPACITY: usize = 10;

pub struct Tray {
    rolls: VecDeque<Roll>,
}

impl Tray {
    pub fn new() -> Self {
        Tray {
            rolls: VecDeque::new()
        }
    }

    // Take a roll command and return the fully formatted result string (or an error)
    pub fn process_roll_command(&mut self, roll_command: &str, roll_comment: &str, roller: &str) -> Result<(RollValue, String), SixballError> {
        let new_roll = self.add_roll_from_command(roll_command, roll_comment, roller)?;

        let final_result = new_roll.result().clone();
        let compact_breakdown = format!("{}", new_roll);

        Ok((final_result, compact_breakdown))
    }

    pub fn rolls(&self) -> &VecDeque<Roll> {
        &self.rolls
    }

    pub fn get_newest_roll(&self) -> Result<&Roll, RollError> {
        let get_roll_result = match self.rolls.back() {
            Some(roll) => Ok(roll),
            None => Err(RollError::RetrieveError("Error retrieving latest roll from tray: Roll queue is empty".to_owned()))
        };

        get_roll_result
    }

    pub fn get_newest_roll_mut(&mut self) -> Result<&mut Roll, RollError> {
        let get_roll_result = match self.rolls.back_mut() {
            Some(roll) => Ok(roll),
            None => Err(RollError::RetrieveError("Error retrieving latest roll from tray: Roll queue is empty".to_owned()))
        };

        get_roll_result
    }

    // Take the command, turn it into a roll, add that to the tray, and return the infix expression that should be passed to the calculator
    fn add_roll_from_command(&mut self, roll_command: &str, roll_comment: &str, roller: &str) -> Result<&Roll, RollError> {
         // If Rolls queue is full, remove the oldest element
        while self.rolls.len() >= CAPACITY { self.rolls.pop_front(); }

        // Make a new empty roll
        let new_roll = Roll::new(roll_command, roll_comment, roller)?;

        // Add new roll to tray
        self.rolls.push_back(new_roll);

        self.get_newest_roll()
    }

    pub fn reroll_latest(&mut self) -> Result<&Roll, RollError> {
        let latest_roll = self.get_newest_roll_mut()?;
        latest_roll.reroll_all();
        Ok(latest_roll)
    }
}
