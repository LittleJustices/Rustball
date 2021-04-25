use rand::{thread_rng, Rng};

#[derive(Debug)]
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

    pub fn set(&mut self, value: u8) {
        if value == 0 {
            self.result = 1
        } else if value > self.sides {
            self.result = self.sides
        } else {
            self.result = value
        }
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
}