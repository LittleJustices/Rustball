use lazy_static::lazy_static;
use regex::Regex;

const DICE_MATCH_STRING: &str = r"(?P<number>\d+)d(?P<sides>\d+)\s*(?:k?(?P<keep>[l|h]?)(?P<keepamt>\d*))";

lazy_static!{
    pub static ref DICE_MATCH_RE: Regex = Regex::new(DICE_MATCH_STRING).expect("Failed to compile dice expression regex!");
}