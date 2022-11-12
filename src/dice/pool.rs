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
    numbers: Vec<u8>,
    sides: Vec<u8>,
    dice: Vec<Die>,
}

impl Pool {
    pub fn new(number: u8, sides: u8) -> Self {
        let mut dice = Vec::<Die>::new();

        for _ in 0..number {
            let die = Die::roll(sides);
            dice.push(die);
        }
        // Don't need to order by sides here because there is only one type of dice

        let numbers = vec![number];
        let sides = vec![sides];

        Pool { numbers, sides, dice }
    }

    pub fn new_from_arrays(number: &[u8], sides: &[u8]) -> Self {
        let mut dice = vec![];

        for (&n, &s) in number.iter().zip(sides.iter()) {
            for _ in 0..n {
                dice.push(Die::roll(s));
            }
        }
        // Order by sides. This lets the rest of the code assume that pools will always be ordered from smallest to biggest die
        // (But unordered within each die size)
        dice.sort_by(|d, e| d.sides.cmp(&e.sides));

        Pool { numbers: number.to_vec(), sides: sides.to_vec(), dice }
    }

    pub fn new_dice_array(number: u8, sides: &[u8]) -> Self {
        let number_arr = vec![number; sides.len()];

        Pool::new_from_arrays(&number_arr, sides)
    }

    pub fn new_numbers_array(number: &[u8], sides: u8) -> Self {
        let number_single = number.iter().fold(0, |n, s| n + s);

        Pool::new(number_single, sides)
    }

    pub fn new_from_dice(dice: &[Die]) -> Self {
        let mut dice = dice.to_vec();
        // Order by sides. This lets the rest of the code assume that pools will always be ordered from smallest to biggest die
        // (But unordered within each die size)
        dice.sort_by(|d, e| d.sides.cmp(&e.sides));

        let mut numbers = vec![];
        let mut sides = vec![];

        let mut n = 0;
        for die in &dice {
            if !sides.contains(&die.sides) {
                if n != 0 { numbers.push(n); }
                sides.push(die.sides);
                n = 1;
            } else {
                n += 1;
            }
        }
        numbers.push(n);

        Pool { numbers, sides, dice }
    }

    pub fn dice(&self) -> &Vec<Die> {
        &self.dice
    }

    pub fn total_number(&self) -> u8 {
        self.dice().len() as u8
    }

    pub fn numbers(&self) -> &[u8] {
        &self.numbers
    }

    pub fn sides(&self) -> &[u8] {
        &self.sides
    }

    pub fn sides_max(&self) -> u8 {
        *self.sides.iter().max().unwrap_or(&0)
    }

    pub fn total(&self) -> u16 {
        // For now, this just returns the sum. In the future it will decide whether to sum, count successes, something else...
        self.sum_sides()
    }

    fn sum_sides(&self) -> u16 {
        self.dice.iter().fold(0, |sum, die| sum + die.result as u16)
    }

    pub fn add(&self, other: &Pool) -> Pool {
        let mut new_dice = self.dice.clone();
        new_dice.extend_from_slice(&other.dice);
        Pool::new_from_dice(&new_dice)
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

    pub fn explode_n(&self, n: u8, recursive: bool) -> Result<Vec<Self>, RollError> {
        if recursive && self.sides_max() == 1 { return Err(RollError::BlockedExplosionError); }
        let mut exploded_pools = vec![];
        exploded_pools.push(self.clone());

        let mut dice_to_explode = self.clone().dice;
        while dice_to_explode.len() > 0 {
            let mut new_dice = vec![];
            for die in dice_to_explode.iter().filter(|d| d.equals(n)) {
                new_dice.push(die.explode());
            }
            exploded_pools.push(Pool::new_from_dice(&new_dice));
            if !recursive { break; }
            dice_to_explode = new_dice;
        }

        Ok(exploded_pools)
    }

    pub fn explode_n_additive(&self, n: u8, recursive: bool) -> Result<Vec<Self>, RollError> {
        let mut exploded_pools = self.explode_n(n, recursive)?;
        let mut result_vector = exploded_pools.clone();

        while exploded_pools.len() >= 2 {
            let mut explosions = exploded_pools.pop().unwrap_or(Pool::new(0, 0));
            for die in exploded_pools.last_mut().unwrap_or(&mut Pool::new(0, 0)).dice.iter_mut().rev().filter(|d| d.equals(n)) {
                let exploded_die = explosions.dice.pop().unwrap_or(Die { sides: 0, result: 0});
                die.set(die.result + exploded_die.result);
            }
        }

        result_vector.push(exploded_pools.pop().unwrap());

        Ok(result_vector)
    }

    pub fn explode_specific(&self, range: &[u8], recursive: bool) -> Result<Vec<Self>, RollError> {
        if recursive && range.len() > (self.sides_max() / 2).into() { return  Err(RollError::BlockedExplosionError); }
        let mut exploded_pools = vec![];
        exploded_pools.push(self.clone());

        let mut dice_to_explode = self.dice.clone();
        while dice_to_explode.len() > 0 {
            let mut new_dice = vec![];
            for die in dice_to_explode.iter().filter(|d| d.is_in(range)) {
                new_dice.push(die.explode());
            }
            exploded_pools.push(Pool::new_from_dice(&new_dice));
            if !recursive { break; }
            dice_to_explode = new_dice;
        }

        Ok(exploded_pools)
    }

    pub fn explode_specific_additive(&self, range: &[u8], recursive: bool) -> Result<Vec<Self>, RollError> {
        let mut exploded_pools = self.explode_specific(range, recursive)?;
        let mut result_vector = exploded_pools.clone();

        while exploded_pools.len() >= 2 {
            let mut explosions = exploded_pools.pop().unwrap_or(Pool::new(0, 0));
            for die in exploded_pools.last_mut().unwrap_or(&mut Pool::new(0, 0)).dice.iter_mut().rev().filter(|d| d.is_in(range)) {
                let exploded_die = explosions.dice.pop().unwrap_or(Die { sides: 0, result: 0});
                die.set(die.result + exploded_die.result);
            }
        }

        result_vector.push(exploded_pools.pop().unwrap());

        Ok(result_vector)
    }

    pub fn keep_exact(&self, range: &[u8]) -> Self {
        let mut kept_dice = vec![];
        for die in self.dice.iter().filter(|d| d.is_in(range)) {
            kept_dice.push(*die);
        }

        Pool::new_from_dice(&kept_dice)
    }

    pub fn keep_highest(&self, argument: u8) -> Self {
        let mut dice_sorted = self.dice.clone();
        dice_sorted.sort_by(|d, e| d.result.cmp(&e.result));

        let min_index = if argument > self.total_number() { 0 } else { (self.total_number() - argument) as usize };

        Pool::new_from_dice(&dice_sorted[min_index..])
    }

    pub fn keep_lowest(&self, argument: u8) -> Self {
        let mut dice_sorted = self.dice.clone();
        dice_sorted.sort_by(|d, e| d.result.cmp(&e.result));

        let max_index = if argument > self.total_number() { self.total_number() as usize } else { argument as usize };

        Pool::new_from_dice(&dice_sorted[..max_index])
    }

    #[allow(dead_code)]
    pub fn reroll_all(&mut self) {
        for die in self.dice.iter_mut() {
            die.reroll();
        }
    }

    pub fn reroll_n(&mut self, n: u8) -> Pool {
        let mut new_rolls = vec![];
        for die in self.dice.iter_mut().filter(|d| d.equals(n)) {
            die.reroll();
            new_rolls.push(*die);
        }
        Pool { dice: new_rolls, ..self.clone() }
    }

    pub fn reroll_n_better(&mut self, n: u8) -> Pool {
        let mut new_rolls = vec![];
        for die in self.dice.iter_mut().filter(|d| d.equals(n)) {
            new_rolls.push(die.reroll_better());
        }
        Pool { dice: new_rolls, ..self.clone() }
    }

    pub fn reroll_n_worse(&mut self, n: u8) -> Pool {
        let mut new_rolls = vec![];
        for die in self.dice.iter_mut().filter(|d| d.equals(n)) {
            new_rolls.push(die.reroll_worse());
        }
        Pool { dice: new_rolls, ..self.clone() }
    }

    pub fn reroll_n_recursive(&mut self, n: u8) -> Pool {
        let mut new_rolls = vec![];
        for die in self.dice.iter_mut().filter(|d| d.equals(n)) {
            die.reroll_excluding_single(n);
            new_rolls.push(*die);
        }
        Pool { dice: new_rolls, ..self.clone() }
    }

    pub fn reroll_specific(&mut self, range: &[u8]) -> Pool {
        let mut new_rolls = vec![];
        for die in self.dice.iter_mut().filter(|d| d.is_in(range)) {
            die.reroll();
            new_rolls.push(*die);
        }
        Pool { dice: new_rolls, ..self.clone() }
    }

    pub fn reroll_specific_better(&mut self, range: &[u8]) -> Pool {
        let mut new_rolls = vec![];
        for die in self.dice.iter_mut().filter(|d| d.is_in(range)) {
            die.reroll_better();
            new_rolls.push(*die);
        }
        Pool { dice: new_rolls, ..self.clone() }
    }

    pub fn reroll_specific_worse(&mut self, range: &[u8]) -> Pool {
        let mut new_rolls = vec![];
        for die in self.dice.iter_mut().filter(|d| d.is_in(range)) {
            die.reroll_worse();
            new_rolls.push(*die);
        }
        Pool { dice: new_rolls, ..self.clone() }
    }

    pub fn reroll_specific_recursive(&mut self, range: &[u8]) -> Pool {
        let mut new_rolls = vec![];
        for die in self.dice.iter_mut().filter(|d| d.is_in(range)) {
            die.reroll_excluding_range(range);
            new_rolls.push(*die);
        }
        Pool { dice: new_rolls, ..self.clone() }
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
        match self.dice.len() {
            0 => write!(f, "[No dice]"),
            _ => {
                let mut results = format!("{}", self.dice[0].result);
                for i in 1..self.dice.len() {
                    results = format!("{}, {}", results, self.dice[i].result)
                }
                write!(f, "[{}]", results)
            }
        }
    }
}

impl FromStr for Pool {
    type Err = RollError;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        // TODO: Actually implement this
        Err(RollError::PlaceholderError)
    }
}
