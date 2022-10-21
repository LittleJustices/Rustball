use std::{
    collections::HashMap,
    str::FromStr
};

use super::{
    dice_errors::RollError,
    die::Die,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GeneSymbol {
    Success,
    Advantage,
    Triumph,
    Failure,
    Threat,
    Despair,
    Blank,
}

impl GeneSymbol {
    pub fn boost(die: Die) -> Vec<Self> {
        match die.result {
            3 => vec![GeneSymbol::Success],
            4 => vec![GeneSymbol::Success, GeneSymbol::Advantage],
            5 => vec![GeneSymbol::Advantage, GeneSymbol::Advantage],
            6 => vec![GeneSymbol::Advantage],
            _ => vec![GeneSymbol::Blank],
        }
    }
    
    pub fn setback(die: Die) -> Vec<Self> {
        match die.result {
            3 => vec![GeneSymbol::Failure],
            4 => vec![GeneSymbol::Failure],
            5 => vec![GeneSymbol::Threat],
            6 => vec![GeneSymbol::Threat],
            _ => vec![GeneSymbol::Blank],
        }
    }
    
    pub fn ability(die: Die) -> Vec<Self> {
        match die.result {
            2 | 3 => vec![GeneSymbol::Success],
            4 => vec![GeneSymbol::Success, GeneSymbol::Success],
            5 | 6 => vec![GeneSymbol::Advantage],
            7 => vec![GeneSymbol::Success, GeneSymbol::Advantage],
            8 => vec![GeneSymbol::Advantage, GeneSymbol::Advantage],
            _ => vec![GeneSymbol::Blank],
        }
    }
    
    pub fn difficulty(die: Die) -> Vec<Self> {
        match die.result {
            2 => vec![GeneSymbol::Failure],
            3 => vec![GeneSymbol::Failure, GeneSymbol::Failure],
            4 | 5 | 6 => vec![GeneSymbol::Threat],
            7 => vec![GeneSymbol::Threat, GeneSymbol::Threat],
            8 => vec![GeneSymbol::Failure, GeneSymbol::Threat],
            _ => vec![GeneSymbol::Blank],
        }
    }
    
    pub fn proficiency(die: Die) -> Vec<Self> {
        match die.result {
            2 | 3 => vec![GeneSymbol::Success],
            4 | 5 => vec![GeneSymbol::Success, GeneSymbol::Success],
            6 => vec![GeneSymbol::Advantage],
            7 | 8 | 9 => vec![GeneSymbol::Success, GeneSymbol::Advantage],
            10 | 11 => vec![GeneSymbol::Advantage, GeneSymbol::Advantage],
            12 => vec![GeneSymbol::Triumph],
            _ => vec![GeneSymbol::Blank],
        }
    }
    
    pub fn challenge(die: Die) -> Vec<Self> {
        match die.result {
            2 | 3 => vec![GeneSymbol::Failure],
            4 | 5 => vec![GeneSymbol::Failure, GeneSymbol::Failure],
            6 | 7 => vec![GeneSymbol::Threat],
            8 | 9 => vec![GeneSymbol::Failure, GeneSymbol::Threat],
            10 | 11 => vec![GeneSymbol::Threat, GeneSymbol::Threat],
            12 => vec![GeneSymbol::Despair],
            _ => vec![GeneSymbol::Blank],
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum GenesysDie {
    Boost(Die),
    Setback(Die),
    Ability(Die),
    Difficulty(Die),
    Proficiency(Die),
    Challenge(Die),
}

impl GenesysDie {
    pub fn result(&self) -> Vec<GeneSymbol> {
        match self {
            GenesysDie::Boost(die) => GeneSymbol::boost(*die),
            GenesysDie::Setback(die) => GeneSymbol::setback(*die),
            GenesysDie::Ability(die) => GeneSymbol::ability(*die),
            GenesysDie::Difficulty(die) => GeneSymbol::difficulty(*die),
            GenesysDie::Proficiency(die) => GeneSymbol::proficiency(*die),
            GenesysDie::Challenge(die) => GeneSymbol::challenge(*die),
        }
    }
}

impl FromStr for GenesysDie {
    type Err = RollError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "b" => Ok(GenesysDie::Boost(Die::roll(6))),
            "s" => Ok(GenesysDie::Setback(Die::roll(6))),
            "a" => Ok(GenesysDie::Ability(Die::roll(8))),
            "d" => Ok(GenesysDie::Difficulty(Die::roll(8))),
            "p" => Ok(GenesysDie::Proficiency(Die::roll(12))),
            "c" => Ok(GenesysDie::Challenge(Die::roll(12))),
            _ => Err(RollError::PlaceholderError),
        }
    }
}

#[derive(Debug)]
pub struct GenesysResults {
    pub boost: Vec<Vec<GeneSymbol>>,
    pub setback: Vec<Vec<GeneSymbol>>,
    pub ability: Vec<Vec<GeneSymbol>>,
    pub difficulty: Vec<Vec<GeneSymbol>>,
    pub proficiency: Vec<Vec<GeneSymbol>>,
    pub challenge: Vec<Vec<GeneSymbol>>,
    pub tally: HashMap<GeneSymbol, u8>,
}

impl GenesysResults {
    pub fn new(dice: &[GenesysDie]) -> Self {
        let (
            mut boost,
            mut setback,
            mut ability,
            mut difficulty,
            mut proficiency,
            mut challenge,
            mut total
        ) = (vec![], vec![], vec![], vec![], vec![], vec![], vec![]);
        let mut results_vector = vec![];

        for genesys_die in dice {
            let res = genesys_die.result();
            for symbol in &res { results_vector.push(*symbol) }
            match genesys_die {
                GenesysDie::Boost(_) => boost.push(res),
                GenesysDie::Setback(_) => setback.push(res),
                GenesysDie::Ability(_) => ability.push(res),
                GenesysDie::Difficulty(_) => difficulty.push(res),
                GenesysDie::Proficiency(_) => proficiency.push(res),
                GenesysDie::Challenge(_) => challenge.push(res),
            }
        }

        for symbol in results_vector {
            match symbol {
                GeneSymbol::Success => {
                    match total.iter().rposition(|&s| s == GeneSymbol::Failure) {
                        Some(index) => { total.swap_remove(index); },
                        None => total.push(symbol),
                    }
                },
                GeneSymbol::Advantage => {
                    match total.iter().rposition(|&s| s == GeneSymbol::Threat) {
                        Some(index) => { total.swap_remove(index); },
                        None => total.push(symbol),
                    }
                },
                GeneSymbol::Failure => {
                    match total.iter().rposition(|&s| s == GeneSymbol::Success) {
                        Some(index) => { total.swap_remove(index); },
                        None => total.push(symbol),
                    }
                },
                GeneSymbol::Threat => {
                    match total.iter().rposition(|&s| s == GeneSymbol::Advantage) {
                        Some(index) => { total.swap_remove(index); },
                        None => total.push(symbol),
                    }
                },
                GeneSymbol::Despair | GeneSymbol::Triumph => total.push(symbol),
                GeneSymbol::Blank => continue,
            }
        }

        let mut tally = HashMap::new();
        for symbol in total {
            tally.entry(symbol).and_modify(|counter| *counter += 1).or_insert(1);
        }

        GenesysResults { boost, setback, ability, difficulty, proficiency, challenge, tally }
    }
}

impl std::fmt::Display for GenesysResults {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.tally.len() == 0 {
            return write!(f, "No dice rolled");
        }

        let tally_str = self.tally.iter().fold(
            String::new(), |s, (key, val)| format!("{}{} {:?}, ", s, val, key)
        );

        let mut details_str = String::new();
        if self.boost.len() != 0 {
            details_str = format!("{}Boost: {:?}; ", details_str, self.boost);
        }
        if self.setback.len() != 0 {
            details_str = format!("{}Setback: {:?}; ", details_str, self.setback);
        }
        if self.ability.len() != 0 {
            details_str = format!("{}Ability: {:?}; ", details_str, self.ability);
        }
        if self.difficulty.len() != 0 {
            details_str = format!("{}Difficulty: {:?}; ", details_str, self.difficulty);
        }
        if self.proficiency.len() != 0 {
            details_str = format!("{}Proficiency: {:?}; ", details_str, self.proficiency);
        }
        if self.challenge.len() != 0 {
            details_str = format!("{}Challenge: {:?}; ", details_str, self.challenge);
        }
        if details_str.len() == 0 {
            details_str = String::from("No dice");
        }

        write!(f, "{}\n{}", tally_str.trim_end_matches(", "), details_str.trim_end_matches("; "))
    }
}
