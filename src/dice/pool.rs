use super::die::Die;

#[derive(Debug)]
pub struct Pool {
    dice: Vec<Die>,
}

impl Pool {
    pub fn new(number: u8, sides: u8) -> Self {
        let mut dice = Vec::<Die>::new();

        for _ in 0..number {
            let die = Die::roll(sides);
            dice.push(die);
        }

        Pool { dice }
    }
}