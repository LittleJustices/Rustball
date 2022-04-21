use lazy_static::lazy_static;
use regex::Regex;
use std::collections::VecDeque;
use super::dice_errors::RollError;
use super::roll::Roll;
use crate::math::calculator;

const DICE_MATCH_STRING: &str = r"(?P<number>\d+)d(?P<sides>\d+)\s*(?:k?(?P<keep>[l|h]?)(?P<keepamt>\d*))";
const CAPACITY: usize = 1;

lazy_static!{
    static ref DICE_MATCH_RE: Regex = Regex::new(DICE_MATCH_STRING).expect("Failed to compile dice expression regex!");
}

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

    // Take the command, turn it into a roll, add that to the tray, and return the infix expression that should be passed to the calculator
    fn add_roll_fom_command(&mut self, roll_command: &str) -> Result<(String, String), RollError> {
         // If Rolls queue is full, remove the oldest element
        while self.rolls.len() >= CAPACITY { self.rolls.pop_front(); }

        // Command to be passed to math
        let mut math_command = roll_command.to_owned();

        // Make a new empty roll
        let mut new_roll = Roll::new(roll_command.to_owned());

        // For each capture in the roll command, add a dicepool constructed from that capture to the roll
        for captures in DICE_MATCH_RE.captures_iter(roll_command) {
            let number = &captures["number"].parse::<u8>()?;
            let sides = &captures["sides"].parse::<u8>()?;
            let pool_total = new_roll.add_pool(*number, *sides);

            math_command = DICE_MATCH_RE.replace(&math_command, pool_total.to_string()).to_string();
        }

        let roll_breakdown = format!("{}", new_roll);
        // Add new roll to tray
        self.rolls.push_back(new_roll);

        Ok((math_command, roll_breakdown))
    }
}