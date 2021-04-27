use rand::{thread_rng, Rng};

#[derive(Debug, PartialEq)]
pub struct Die {
    sides: u8,
    result: u8,
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

    pub fn set(&mut self, value: u8) {
        self.result = value
    }

    pub fn explode(&self) -> Die {
        Die::roll(self.sides)
    }

    pub fn equals(&self, value: u8) -> bool {
        self.result == value
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