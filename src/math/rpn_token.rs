use std::str::FromStr;
use super::math_errors::MathError;

#[derive(Clone, Debug, PartialEq)]
pub enum RpnToken {
    Number(f64),
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    RParen,
    LParen,
}

impl RpnToken {
    pub fn precedence(&self) -> u8 {
        match self {
            RpnToken::Add | RpnToken::Sub => 4,
            RpnToken::Mul | RpnToken::Div => 5,
            RpnToken::Pow => 6,
            _ => 0
        }
    }
}

impl FromStr for RpnToken {
    type Err = MathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let token: Result<RpnToken, MathError> = match s.trim() {
            "+" => Ok(RpnToken::Add),
            "-" => Ok(RpnToken::Sub),
            "*" | "x" => Ok(RpnToken::Mul),
            "/" => Ok(RpnToken::Div),
            "^" | "**" => Ok(RpnToken::Pow),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_str() {
        let strings_to_parse = ["+", " -", "* ", " x ", "/", "  ^  ", "\t**", ")", "(", "1", "2.0", "-15", "apple"];

        assert_eq!(RpnToken::Add, strings_to_parse[0].parse().unwrap());
        assert_eq!(RpnToken::Sub, strings_to_parse[1].parse().unwrap());
        assert_eq!(RpnToken::Mul, strings_to_parse[2].parse().unwrap());
        assert_eq!(RpnToken::Mul, strings_to_parse[3].parse().unwrap());
        assert_eq!(RpnToken::Div, strings_to_parse[4].parse().unwrap());
        assert_eq!(RpnToken::Pow, strings_to_parse[5].parse().unwrap());
        assert_eq!(RpnToken::Pow, strings_to_parse[6].parse().unwrap());
        assert_eq!(RpnToken::RParen, strings_to_parse[7].parse().unwrap());
        assert_eq!(RpnToken::LParen, strings_to_parse[8].parse().unwrap());
        assert_eq!(RpnToken::Number(1.0), strings_to_parse[9].parse().unwrap());
        assert_eq!(RpnToken::Number(2.0), strings_to_parse[10].parse().unwrap());
        assert_eq!(RpnToken::Number(-15.0), strings_to_parse[11].parse().unwrap());
        assert!(strings_to_parse[12].parse::<RpnToken>().is_err());
    }
}
