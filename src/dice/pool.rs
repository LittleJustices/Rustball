use super::{
    die::Die,
    dice_errors::RollError,
    roll_token::Keep,
};
use std::{
    fmt,
    str::FromStr,
};

#[derive(Debug, PartialEq)]
pub struct Pool {
    number: u8,
    sides: u8,
    keep: Keep,
    dice: Vec<Die>,
}

impl Pool {
    pub fn new(number: u8, sides: u8, keep: Keep) -> Self {
        let mut dice = Vec::<Die>::new();

        for _ in 0..number {
            let die = Die::roll(sides);
            dice.push(die);
        }

        Pool { number, sides, keep, dice }
    }

    pub fn total(&self) -> u16 {
        // For now, this just returns the sum. In the future it will decide whether to sum, count successes, something else...
        self.sum_sides()
    }

    fn sum_sides(&self) -> u16 {
        if self.keep != Keep::All {
            return self.keep_dice(&self.keep).iter().fold(0, |sum, die| sum + die.result as u16);
        }
        self.dice.iter().fold(0, |sum, die| sum + die.result as u16)
    }

    fn keep_dice(&self, keep: &Keep) -> Vec<&Die> {
        let mut kept_dice: Vec<&Die> = self.dice.iter().collect();

        kept_dice.sort_unstable();
        match keep {
            Keep::Low(keepamt) => {
                let max_index = if keepamt > &self.number { self.number as usize } else { *keepamt as usize };
                return kept_dice[..max_index].to_vec();
            }
            Keep::High(keepamt) => {
                let min_index = if keepamt > &self.number { 0 } else { (self.number - *keepamt) as usize };
                return kept_dice[min_index..].to_vec();
            },
            Keep::All => return kept_dice,
        }
    }

    pub fn reroll(&mut self) {
        for die in self.dice.iter_mut() {
            die.reroll();
        }
    }

    #[allow(dead_code)]
    fn reroll_n(&mut self, n: u8) {
        for die in self.dice.iter_mut().filter(|d| d.equals(n)) {
            die.reroll();
        }
    }

    #[allow(dead_code)]
    fn reroll_n_or_less(&mut self, n: u8) {
        for die in self.dice.iter_mut().filter(|d| d.equal_or_less(n)) {
            die.reroll();
        }
    }

    #[allow(dead_code)]
    fn reroll_n_or_higher(&mut self, n: u8) {
        for die in self.dice.iter_mut().filter(|d| d.equal_or_greater(n)) {
            die.reroll();
        }
    }
}

impl fmt::Display for Pool {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut results = format!("{}", self.dice[0].result);
        for i in 1..self.dice.len() {
            results = format!("{}, {}", results, self.dice[i].result)
        }
        write!(f, "{}d{} -> [{}]", self.number, self.sides, results)
    }
}

impl FromStr for Pool {
    type Err = RollError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        todo!()
    }
}
