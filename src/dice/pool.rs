use super::die::Die;
use std::fmt;

#[derive(Debug)]
pub struct Pool {
    number: u8,
    sides: u8,
    keep_low: bool,
    kept_dice: u8,
    dice: Vec<Die>,
}

impl Pool {
    pub fn new(number: u8, sides: u8, keep_low: bool, keepamt: u8) -> Self {
        let mut dice = Vec::<Die>::new();

        for _ in 0..number {
            let die = Die::roll(sides);
            dice.push(die);
        }

        // Constrain number of kept dice to no more than number of rolled dice
        let kept_dice = if keepamt > number {
            number
        } else {
            keepamt
        };

        Pool { number, sides, keep_low, kept_dice, dice }
    }

    pub fn total(&self) -> u16 {
        // For now, this just returns the sum. In the future it will decide whether to sum, count successes, something else...
        self.sum_sides()
    }

    fn sum_sides(&self) -> u16 {
        let mut total = 0;
        for die in &self.kept_dice() {
            total += die.result as u16;
        }
        total
    }

    fn kept_dice(&self) -> Vec<Die> {
        if self.kept_dice == 0 {
            return self.dice.clone();
        }

        let mut dice_sorted = self.dice.clone();
        dice_sorted.sort_unstable();
        let mut kept_pool = Vec::<Die>::new();
        match self.keep_low {
            true => {
                for die in dice_sorted.iter().take(self.kept_dice.into()) {
                    kept_pool.push(*die);
                };
            },
            false => {
                for die in dice_sorted.iter().rev().take(self.kept_dice.into()) {
                    kept_pool.push(*die);
                };
            },
        };

        kept_pool
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