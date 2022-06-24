use chrono::prelude::*;
use std::fmt;
use super::dice_re::DICE_MATCH_RE;
use super::dice_errors::RollError;
use super::pool::{Keep, Pool};

#[derive(Debug)]
pub struct Roll {
    command: String,
    dicepools: Vec<Pool>,
    roller: String,
    timestamp: DateTime<Utc>,
}

impl Roll {
    pub fn new(command: &str) -> Result<Self, RollError> {
        let roller = "Placeholder name".to_owned();
        let timestamp = Utc::now();
        let mut dicepools = Vec::new();
        for captures in DICE_MATCH_RE.captures_iter(command) {
            let number = captures["number"].parse::<u8>()?;
            let sides = captures["sides"].parse::<u8>()?;

            let keep = match &captures["keep"] {
                "l" => Keep::Low(captures["keepamt"].parse::<u8>()?),
                _ => {
                    match &captures["keepamt"] {
                        "" => Keep::All,
                        _ => Keep::High(captures["keepamt"].parse::<u8>()?),
                    }
                }
            };
            dicepools.push(Pool::new(number, sides, keep));
        }
        Ok(Roll { command: command.to_string(), dicepools, roller, timestamp })
    }

    pub fn math_command(&self) -> String {
        let mut math_command = self.command.clone();
        for pool in &self.dicepools {
            math_command = DICE_MATCH_RE.replace(&math_command, pool.total().to_string()).to_string();
        }
        math_command
    }

    #[allow(dead_code)]
    pub fn reroll_all(&mut self) {
        for pool in self.dicepools.iter_mut() {
            pool.reroll();
        }
    }

    pub fn roller(&self) -> &str {
        &self.roller
    }

    pub fn timestamp(&self) -> DateTime<Utc> {
        self.timestamp
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