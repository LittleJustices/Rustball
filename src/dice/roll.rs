use std::fmt;
use super::dice_re::DICE_MATCH_RE;
use super::dice_errors::RollError;
use super::pool::Pool;

#[derive(Debug)]
pub struct Roll {
    command: String,
    dicepools: Vec<Pool>,
}

impl Roll {
    pub fn new(command: String) -> Result<Self, RollError> {
        let mut dicepools = Vec::new();
        for captures in DICE_MATCH_RE.captures_iter(&command) {
            let number = &captures["number"].parse::<u8>()?;
            let sides = &captures["sides"].parse::<u8>()?;
            dicepools.push(Pool::new(*number, *sides));
        }
        Ok(Roll { command, dicepools })
    }

    pub fn math_command(&self) -> String {
        let mut math_command = self.command.clone();
        for pool in &self.dicepools {
            math_command = DICE_MATCH_RE.replace(&math_command, pool.total().to_string()).to_string();
        }
        math_command
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

mod tests {
    // use super::*;

    #[test]
    fn roll_from_string() {
        unimplemented!();
    }
}