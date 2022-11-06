use rand::{thread_rng, Rng};
use std::fmt;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Die {
    pub sides: u8,
    pub result: u8,
}

impl Die {
    pub fn roll(sides: u8) -> Die {
        let mut rng = thread_rng();
        let result = rng.gen_range(1..=sides);
        Die { sides, result }
    }

    pub fn reroll(&mut self) {
        let mut rng = thread_rng();
        self.result = rng.gen_range(1..=self.sides);
    }

    pub fn reroll_better(&mut self) -> Die {
        let new_die = self.explode();
        if new_die > *self { self.set(new_die.result) };
        new_die
    }

    pub fn reroll_worse(&mut self) -> Die {
        let new_die = self.explode();
        if new_die < *self { self.set(new_die.result) };
        new_die
    }

    pub fn reroll_excluding_single(&mut self, excluded: u8) {
        let mut rng = thread_rng();
        let mut new_result = rng.gen_range(1..self.sides);
        if new_result >= excluded { new_result += 1; }
        self.set(new_result);
    }

    pub fn reroll_excluding_range(&mut self, excluded: &[u8]) {
        let mut rng = thread_rng();
        let possible_values: Vec<u8> = (1..=self.sides).filter(|x| !excluded.contains(x)).collect();
        if possible_values.len() == 0 { return; }   // Leave the die alone if there are no possible values to reroll to
        let random_index = rng.gen_range(0..possible_values.len());
        self.set(possible_values[random_index]);
    }

    pub fn set(&mut self, value: u8) {
        self.result = value
    }

    pub fn explode(&self) -> Die {
        Die::roll(self.sides)
    }

    pub fn equals(&self, value: u8) -> bool {
        self.result == value
    }

    pub fn is_in(&self, range: &[u8]) -> bool {
        range.contains(&self.result)
    }

    pub fn equal_or_greater(&self, target: u8) -> bool {
        self.result >= target
    }

    pub fn equal_or_less(&self, target: u8) -> bool {
        self.result <= target
    }

    pub fn count_successes(&self, tns: &[u8]) -> u8 {
        tns[(self.result-1) as usize]
    }
}

impl fmt::Display for Die {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.result)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn set_test() {
        let mut die = Die::roll(20);
        die.set(20);
        assert_eq!(Die{sides: 20, result: 20}, die);
    }
    
    #[test]
    fn test_eq() {
        let die = Die {sides: 20, result: 10};
        assert!(die.equals(10));
    }

    #[test]
    fn test_geq() {
        let die = Die {sides: 20, result: 15};
        assert!(die.equal_or_greater(13));
    }

    #[test]
    fn test_leq() {
        let die = Die {sides: 20, result: 5};
        assert!(die.equal_or_less(8));
    }

    #[test]
    fn test_successes() {
        let sux_map = [0, 0, 0, 0, 0, 0, 1, 1, 1, 2];
        let sides = 10;
        let pool = [
            Die { sides, result: 1 },
            Die { sides, result: 3 },
            Die { sides, result: 6 },
            Die { sides, result: 7 },
            Die { sides, result: 10 },
        ];
        
        let mut successes = 0;
        for die in pool.iter() {
            successes += die.count_successes(&sux_map);
        }

        assert_eq!(3, successes)
    }
}
