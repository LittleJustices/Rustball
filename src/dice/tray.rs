use std::collections::VecDeque;
use super::dice_errors::RollError;
use super::dice_re::DICE_MATCH_RE;
use super::roll::Roll;
use crate::math::calculator;
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
    pub fn process_roll_command(&mut self, roll_command: &str) -> Result<(f64, String), SixballError> {
        // Check if there is a dice expression in the command
        if !DICE_MATCH_RE.is_match(roll_command) {
            // If no dice, treat it as a mathematical expression and toss it to the calculator
            let calc_result = calculator::evaluate(roll_command)?;
            return Ok((calc_result, "No dice rolled".to_owned()));
        }

        // Interim because it might be a math expression that we have to resolve
        let (interim_result, compact_breakdown) = self.add_roll_from_command(roll_command)?;

        let final_result = calculator::evaluate(&interim_result.trim())?;

        Ok((final_result, compact_breakdown))
    }

    pub fn rolls(&self) -> &VecDeque<Roll> {
        &self.rolls
    }

    #[allow(dead_code)]
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
    fn add_roll_from_command(&mut self, roll_command: &str) -> Result<(String, String), RollError> {
         // If Rolls queue is full, remove the oldest element
        while self.rolls.len() >= CAPACITY { self.rolls.pop_front(); }

        // Make a new empty roll
        let new_roll = Roll::new(roll_command)?;

        let math_command = new_roll.math_command();
        let roll_breakdown = format!("{}", new_roll);
        // Add new roll to tray
        self.rolls.push_back(new_roll);

        Ok((math_command, roll_breakdown))
    }

    pub fn reroll_latest(&mut self) -> Result<&Roll, RollError> {
        let latest_roll = self.get_newest_roll_mut()?;
        latest_roll.reroll_all();
        Ok(latest_roll)
    }
}
