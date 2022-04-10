use std::fmt;
use super::pool::Pool;

#[derive(Debug)]
pub struct Roll {
    _command: String,
    dicepools: Vec<Pool>,
}

impl Roll {
    pub fn new(_command: String) -> Self {
        Roll { _command, dicepools: Vec::new() }
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

mod tests {
    // use super::*;

    #[test]
    fn roll_from_string() {
        unimplemented!();
    }
}