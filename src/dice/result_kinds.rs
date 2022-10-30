use std::collections::HashMap;
use super::{
    genesymbols::GeneSymbol,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub struct GenesysValue {
    pub tally: HashMap<GeneSymbol, u8>
}

impl GenesysValue {
    pub fn new(symbols: &Vec<Vec<GeneSymbol>>) -> Self {
        let mut tally = HashMap::new();

        for symbol in symbols.iter().flatten() {
            match symbol {
                GeneSymbol::Success => {
                    let cancel = GeneSymbol::Failure;
                    match tally.get(&cancel) {
                        None | Some(0) => { tally.entry(*symbol).and_modify(|counter| *counter += 1).or_insert(1); },
                        Some(_) => { tally.entry(cancel).and_modify(|counter| *counter -= 1).or_insert(1); }
                    }
                },
                GeneSymbol::Advantage => {
                    let cancel = GeneSymbol::Threat;
                    match tally.get(&cancel) {
                        None | Some(0) => { tally.entry(*symbol).and_modify(|counter| *counter += 1).or_insert(1); },
                        Some(_) => { tally.entry(cancel).and_modify(|counter| *counter -= 1).or_insert(1); }
                    }
                },
                GeneSymbol::Failure => {
                    let cancel = GeneSymbol::Success;
                    match tally.get(&cancel) {
                        None | Some(0) => { tally.entry(*symbol).and_modify(|counter| *counter += 1).or_insert(1); },
                        Some(_) => { tally.entry(cancel).and_modify(|counter| *counter -= 1).or_insert(1); }
                    }
                },
                GeneSymbol::Threat => {
                    let cancel = GeneSymbol::Advantage;
                    match tally.get(&cancel) {
                        None | Some(0) => { tally.entry(*symbol).and_modify(|counter| *counter += 1).or_insert(1); },
                        Some(_) => { tally.entry(cancel).and_modify(|counter| *counter -= 1).or_insert(1); }
                    }
                },
                GeneSymbol::Triumph | GeneSymbol::Despair => { tally.entry(*symbol).and_modify(|counter| *counter += 1).or_insert(1); },
                GeneSymbol::Blank => continue,
            };
        }

        GenesysValue { tally }
    }

    fn to_vector(&self) -> Vec<GeneSymbol> {
        let mut symbols = vec![];

        for (&symbol, &number) in self.tally.iter() {
            for _ in 0..number {
                symbols.push(symbol);
            }
        }

        symbols
    }

    pub fn add(self, other: GenesysValue) -> Self {
        let symbols = vec![self.to_vector(), other.to_vector()];

        GenesysValue::new(&symbols)
    }
}

impl fmt::Display for GenesysValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.tally.len() == 0 {
            return write!(f, "No dice rolled");
        }

        let tally_str = self.tally.iter().fold(
            String::new(), |s, (key, val)| {
                if *val == 0 { s } else {
                    let plural = match val {
                        1 => "",
                        _ => match key {
                            GeneSymbol::Success => "es",
                            _ => "s"
                        }
                    };
                    format!("{}{} {:?}{}, ", s, val, key, plural)
                }
            }
        );

        write!(f, "{}", tally_str.trim_end_matches(", "))
    }
}
