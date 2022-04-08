use regex::Regex;
use std::collections::VecDeque;
use std::str::FromStr;
use super::dice_errors::RollError;
use super::roll::Roll;

const DICE_MATCH_STRING: &str = r"\d+d\d+";
const DICE_SPLIT_STRING: &str = r"d";
const CAPACITY: usize = 1;

pub struct Tray {
    dice_match_re: Regex,
    dice_split_re: Regex,
    rolls: VecDeque<Roll>,
}

impl Tray {
    pub fn new() -> Self {
        Tray {
            dice_match_re: Regex::new(DICE_MATCH_STRING).expect("Failed to compile dice matching regex!"),
            dice_split_re: Regex::new(DICE_SPLIT_STRING).expect("Failed to compile dice splitting regex!"),
            rolls: VecDeque::new()
        }
    }

    pub fn add_roll_from_string(&mut self, roll_command: &str) -> Result<(), RollParseError> {
        while self.rolls.len() >= CAPACITY { self.rolls.pop_front(); } // If Rolls queue is full, remove the oldest element

        let roll_result = Roll::from_str(roll_command);

        let add_to_tray_result = match roll_result {
            Ok(roll) => {
                self.rolls.push_back(roll);
                Ok(())
            },
            Err(why) => Err(why)
        };

        add_to_tray_result
    }

    pub fn get_newest_roll(&self) -> Result<&Roll, RollError> {
        let get_roll_result = match self.rolls.back() {
            Some(roll) => Ok(roll),
            None => Err(RollError::RetrieveError("Error retrieving latest roll from tray: Roll queue is empty".to_owned()))
        };

        get_roll_result
    }
}