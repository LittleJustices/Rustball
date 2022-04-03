use regex::Regex;
use super::roll::Roll;

const DICE_MATCH_STRING: &str = r"\d+d\d+";
const DICE_SPLIT_STRING: &str = r"d";

pub struct Tray {
    dice_match_re: Regex,
    dice_split_re: Regex,
    rolls: Vec<Roll>,
}

impl Tray {
    pub fn new() -> Self {
        Tray {
            dice_match_re: Regex::new(DICE_MATCH_STRING).expect("Failed to compile dice matching regex!"),
            dice_split_re: Regex::new(DICE_SPLIT_STRING).expect("Failed to compile dice splitting regex!"),
            rolls: vec![]
        }
    }
}