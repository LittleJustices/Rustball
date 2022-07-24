use std::str::FromStr;
use super::math_errors::MathError;

#[derive(Clone, Debug, PartialEq)]
pub enum RpnToken {
    Number(f64),
    Operator(Operator),
    RParen,
    LParen,
}

impl RpnToken {
    pub fn precedence(&self) -> u8 {
        match self {
            RpnToken::Operator(operator) => operator.precedence(),
            _ => 0
        }
    }
}

impl FromStr for RpnToken {
    type Err = MathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let token: Result<RpnToken, MathError> = match s.trim() {
            "+" => Ok(RpnToken::Operator(Operator::Add)),
            "-" => Ok(RpnToken::Operator(Operator::Sub)),
            "*" | "x" => Ok(RpnToken::Operator(Operator::Mul)),
            "/" => Ok(RpnToken::Operator(Operator::Div)),
            "^" | "**" => Ok(RpnToken::Operator(Operator::Pow)),
            ")" | "]" | "}" => Ok(RpnToken::RParen),
            "(" | "[" | "{" => Ok(RpnToken::LParen),
            other => match other.parse() {
                Ok(num) => Ok(RpnToken::Number(num)),
                Err(_) => Err(MathError::SymbolError(other.to_string()))
            }
        };
        
        token
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum Operator {
    Add,
    Sub,
    Mul,
    Div,
    Pow,
}

impl Operator {
    pub fn precedence(&self) -> u8 {
        match self {
            Operator::Add | Operator::Sub => 4,
            Operator::Mul | Operator::Div => 5,
            Operator::Pow => 6,
        }
    }

    pub fn left_associative(&self) -> bool {
        match self {
            Operator::Pow => false,
            _ => true
        }
    }

    pub fn apply(&self, left: f64, right: f64) -> f64 {
        match self {
            Operator::Add => left + right,
            Operator::Sub => left - right,
            Operator::Mul => left * right,
            Operator::Div => left / right,
            Operator::Pow => left.powf(right),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let strings_to_parse = ["+", " -", "* ", " x ", "/", "  ^  ", "\t**", ")", "(", "1", "2.0", "-15", "apple"];

        assert_eq!(RpnToken::Operator(Operator::Add), strings_to_parse[0].parse().unwrap());
        assert_eq!(RpnToken::Operator(Operator::Sub), strings_to_parse[1].parse().unwrap());
        assert_eq!(RpnToken::Operator(Operator::Mul), strings_to_parse[2].parse().unwrap());
        assert_eq!(RpnToken::Operator(Operator::Mul), strings_to_parse[3].parse().unwrap());
        assert_eq!(RpnToken::Operator(Operator::Div), strings_to_parse[4].parse().unwrap());
        assert_eq!(RpnToken::Operator(Operator::Pow), strings_to_parse[5].parse().unwrap());
        assert_eq!(RpnToken::Operator(Operator::Pow), strings_to_parse[6].parse().unwrap());
        assert_eq!(RpnToken::RParen, strings_to_parse[7].parse().unwrap());
        assert_eq!(RpnToken::LParen, strings_to_parse[8].parse().unwrap());
        assert_eq!(RpnToken::Number(1.0), strings_to_parse[9].parse().unwrap());
        assert_eq!(RpnToken::Number(2.0), strings_to_parse[10].parse().unwrap());
        assert_eq!(RpnToken::Number(-15.0), strings_to_parse[11].parse().unwrap());
        assert!(strings_to_parse[12].parse::<RpnToken>().is_err());
    }
}
