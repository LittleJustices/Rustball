use super::{
    die::Die,
    dice_errors::RollError,
};
use std::{
    fmt,
    str::FromStr,
};

#[derive(Clone, Debug, PartialEq)]
pub struct Pool {
    number: u8,
    sides: u8,
    dice: Vec<Die>,
}

impl Pool {
    pub fn new(number: u8, sides: u8) -> Self {
        let mut dice = Vec::<Die>::new();

        for _ in 0..number {
            let die = Die::roll(sides);
            dice.push(die);
        }

        Pool { number, sides, dice }
    }

    #[allow(dead_code)]
    pub fn dice(&self) -> &Vec<Die> {
        &self.dice
    }

    pub fn number(&self) -> u8 {
        self.number
    }

    pub fn sides(&self) -> u8 {
        self.sides
    }

    pub fn total(&self) -> u16 {
        // For now, this just returns the sum. In the future it will decide whether to sum, count successes, something else...
        self.sum_sides()
    }

    fn sum_sides(&self) -> u16 {
        self.dice.iter().fold(0, |sum, die| sum + die.result as u16)
    }

    pub fn count_dice_over(&self, target: u8) -> u8 {
        self.dice.iter().filter(|d| d.equal_or_greater(target)).fold(0, |sum, _| sum + 1)
    }

    pub fn count_dice_under(&self, target: u8) -> u8 {
        self.dice.iter().filter(|d| d.equal_or_less(target)).fold(0, |sum, _| sum + 1)
    }

    pub fn count_successes(&self, tns: &[u8]) -> u16 {
        self.dice.iter().fold(0, |sum, die| sum + die.count_successes(tns) as u16)
    }

    pub fn explode_n(&self, n: u8) -> Self {
        let mut exploded_pool = self.clone();
        for die in self.dice.iter().filter(|d| d.equals(n)) {
            exploded_pool.dice.push(die.explode());
        }

        exploded_pool
    }

    pub fn explode_n_additive(&self, n: u8) -> Self {
        let mut dice = vec![];
        for die in &self.dice {
            if die.equals(n) {
                dice.push(die.explode_additive());
            } else {
                dice.push(*die);
            }
        }

        Pool { dice, ..*self }
    }

    pub fn explode_specific(&self, range: &[u8]) -> Self {
        let mut exploded_pool = self.clone();
        for die in self.dice.iter().filter(|d| d.is_in(range)) {
            exploded_pool.dice.push(die.explode());
        }

        exploded_pool
    }

    pub fn explode_specific_additive(&self, range: &[u8]) -> Self {
        let mut dice = vec![];
        for die in &self.dice {
            if die.is_in(range) {
                dice.push(die.explode_additive());
            } else {
                dice.push(*die);
            }
        }

        Pool { dice, ..*self }
    }

    pub fn keep_exact(&self, range: &[u8]) -> Self {
        let mut kept_dice = vec![];
        for die in self.dice.iter().filter(|d| d.is_in(range)) {
            kept_dice.push(*die);
        }

        Pool { dice: kept_dice, ..*self }
    }

    pub fn keep_highest(&self, argument: u8) -> Self {
        let mut dice_sorted = self.dice.clone();
        dice_sorted.sort_unstable();

        let min_index = if argument > self.number { 0 } else { (self.number - argument) as usize };

        Pool { dice: dice_sorted[min_index..].to_vec(), ..*self }
    }

    pub fn keep_lowest(&self, argument: u8) -> Self {
        let mut dice_sorted = self.dice.clone();
        dice_sorted.sort_unstable();

        let max_index = if argument > self.number { self.number as usize } else { argument as usize };

        Pool { dice: dice_sorted[..max_index].to_vec(), ..*self }
    }

    pub fn reroll_all(&mut self) {
        for die in self.dice.iter_mut() {
            die.reroll();
        }
    }

    pub fn reroll_n(&mut self, n: u8) {
        for die in self.dice.iter_mut().filter(|d| d.equals(n)) {
            die.reroll();
        }
    }

    pub fn reroll_n_better(&mut self, n: u8) {
        for die in self.dice.iter_mut().filter(|d| d.equals(n)) {
            die.reroll_better();
        }
    }

    pub fn reroll_n_worse(&mut self, n: u8) {
        for die in self.dice.iter_mut().filter(|d| d.equals(n)) {
            die.reroll_worse();
        }
    }

    pub fn reroll_specific(&mut self, range: &[u8]) {
        for die in self.dice.iter_mut().filter(|d| d.is_in(range)) {
            die.reroll();
        }
    }

    pub fn reroll_specific_better(&mut self, range: &[u8]) {
        for die in self.dice.iter_mut().filter(|d| d.is_in(range)) {
            die.reroll_better();
        }
    }

    pub fn reroll_specific_worse(&mut self, range: &[u8]) {
        for die in self.dice.iter_mut().filter(|d| d.is_in(range)) {
            die.reroll_worse();
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
        write!(f, "[{}]", results)
    }
}

impl FromStr for Pool {
    type Err = RollError;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        // TODO: Actually implement this
        Err(RollError::PlaceholderError)
    }
}
