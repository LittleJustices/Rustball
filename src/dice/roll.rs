use std::fmt;
// use lazy_static::lazy_static;
// use regex::Regex;
// use std::str::FromStr;
use super::{
    // dice_errors::RollError,
    pool::Pool,
};

/* 
lazy_static! {
    static ref DICE_MATCH_RE: Regex = Regex::new(r"\d+d\d+").expect("Failed to compile dice matching regex!");
    static ref DICE_SPLIT_RE: Regex = Regex::new(r"d").expect("Failed to compile dice splitting regex!");
}
*/

#[derive(Debug)]
pub struct Roll {
    command: String,
    dicepools: Vec<Pool>,
}

impl Roll {
    pub fn new(command: String) -> Self {
        Roll { command, dicepools: Vec::new() }
    }

    pub fn add_pool(&mut self, number: u8, sides: u8) -> u16 {
        let new_pool = Pool::new(number, sides);
        let total = new_pool.total();
        self.dicepools.push(new_pool);
        total
    }
}

impl fmt::Display for Roll {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut pools = format!("{}", self.dicepools[0]);
        for i in 1..self.dicepools.len() {
            pools = format!("{}; {}", pools, self.dicepools[i]);
        }
        write!(f, "{}", pools)
    }
}

/* 
impl FromStr for Roll {
    type Err = RollError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if !DICE_MATCH_RE.is_match(s) { return Err(RollError::InputError("Expected argument of the form XdY".to_owned())) }

        let args: Vec<&str> = DICE_SPLIT_RE.split(s).collect();
        let number = args[0].parse::<u8>()?;
        let sides = args[1].parse::<u8>()?;

        Ok(Roll { command: s.to_owned(), dicepools: vec![Pool::new(number, sides)] })
    }
}
*/

mod tests {
    // use super::*;

    #[test]
    fn roll_from_string() {
        unimplemented!();
    }
}