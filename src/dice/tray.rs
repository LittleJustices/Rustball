use std::collections::VecDeque;
use super::dice_errors::RollError;
use super::dice_re::DICE_MATCH_RE;
use super::roll::Roll;
use crate::math::calculator;

const CAPACITY: usize = 1;

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
    pub fn process_roll_command(&mut self, roll_command: &str) -> Result<(String, String), RollError> {
        // Check if there is a dice expression in the command
        if !DICE_MATCH_RE.is_match(roll_command) {
            // If no dice, treat it as a mathematical expression and toss it to the calculator
            let calc_result = match calculator::evaluate(roll_command) {
                Ok(res) => res,
                Err(why) => return Err(RollError::MathError(why))
            };
            return Ok((calc_result, "No dice rolled".to_owned()));
        }

        let interim_result; // Interim because it might be a math expression that we have to resolve
        let compact_breakdown;
        match self.add_roll_fom_command(roll_command) {
            Ok(res) => (interim_result, compact_breakdown) = res,
            Err(why) => return Err(why)
        };

        let final_result;
        match calculator::evaluate(&interim_result.trim()) {
            Ok(res) => final_result = res,
            Err(why) => return Err(RollError::MathError(why))
        };

        Ok((final_result, compact_breakdown))
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
    fn add_roll_fom_command(&mut self, roll_command: &str) -> Result<(String, String), RollError> {
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