use std::collections::VecDeque;
use super::{
    dice_errors::RollError,
    roll::Roll,
};

const CAPACITY: usize = 16;

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
        match number.parse()? {
            repeat @ 1..=CAPACITY => Ok(repeat as u8),
            _ => Err(RollError::PlaceholderError)
        }
    }

    pub fn rolls(&self) -> &VecDeque<Roll> {
        &self.rolls
    }

    pub fn get_newest_roll(&self) -> Result<&Roll, RollError> {
        self.rolls.back().ok_or(RollError::RetrieveError)
    }

    // Take a roll command, resolve it, and return the roll while storing it in the tray
    pub fn add_roll_from_command(&mut self, roll_command: &str, roll_comment: &str, roller: &str) -> Result<&Roll, RollError> {
         // If Rolls queue is full, remove the oldest element
        while self.rolls.len() >= CAPACITY { self.rolls.pop_front(); }

        // Make a new empty roll
        let new_roll = Roll::new(roll_command, roll_comment, roller)?;

        // Add new roll to tray
        self.rolls.push_back(new_roll);

        self.get_newest_roll()
    }

    pub fn reroll_latest(&mut self) -> Result<&Roll, RollError> {
        let latest_roll = self.get_newest_roll()?;
        let new_roll = latest_roll.roll_again()?;
        self.rolls.push_back(new_roll);

        self.get_newest_roll()
    }

    pub fn modify_latest(&mut self, revision_command: &str, revision_comment: &str, reviser: &str) -> Result<&Roll, RollError> {
        let latest_roll = self.get_newest_roll()?;
        let new_roll = latest_roll.revise(revision_command, revision_comment, reviser)?;
        self.rolls.push_back(new_roll);

        self.get_newest_roll()
    }
}
