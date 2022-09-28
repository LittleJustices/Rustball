use super::die::Die;

#[allow(dead_code)]
pub enum GeneSymbol {
    Success,
    Advantage,
    Triumph,
    Failure,
    Threat,
    Despair,
    Blank,
}

#[allow(dead_code)]
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
