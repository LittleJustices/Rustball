use std::collections::VecDeque;
use super::{
    dice_errors::RollError,
    roll::Roll,
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

    pub fn repeat_rolls(number: &str) -> Result<u8, RollError> {
        Ok(number.parse()?)
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

    // Take a roll command, resolve it, and return the roll while storing it in the tray
    pub fn add_roll_from_command(&mut self, roll_command: &str, roll_comment: &str, roller: &str) -> Result<&Roll, SixballError> {
         // If Rolls queue is full, remove the oldest element
        while self.rolls.len() >= CAPACITY { self.rolls.pop_front(); }

        // Make a new empty roll
        let new_roll = Roll::new(roll_command, roll_comment, roller)?;

        // Add new roll to tray
        self.rolls.push_back(new_roll);

        Ok(self.get_newest_roll()?)
    }

    pub fn reroll_latest(&mut self) -> Result<&Roll, RollError> {
        let latest_roll = self.get_newest_roll_mut()?;
        latest_roll.reroll_all();
        Ok(latest_roll)
    }
}
