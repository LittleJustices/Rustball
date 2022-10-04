use std::str::FromStr;

use super::{
    dice_errors::RollError,
    die::Die,
};

#[derive(Debug)]
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
