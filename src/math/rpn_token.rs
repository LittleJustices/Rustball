use std::str::FromStr;
use super::math_errors::MathError;

#[derive(Debug, PartialEq)]
pub enum Token {
    Number(f64),
    Add,
    Sub,
    Mul,
    Div,
    Pow,
    RParen,
    LParen,
}

impl FromStr for Token {
    type Err = MathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let token: Result<Token, MathError> = match s.trim() {
            "+" => Ok(Token::Add),
            "-" => Ok(Token::Sub),
            "*" => Ok(Token::Mul),
            "x" => Ok(Token::Mul),
            "/" => Ok(Token::Div),
            "^" => Ok(Token::Pow),
            "**" => Ok(Token::Pow),
            ")" => Ok(Token::RParen),
            "(" => Ok(Token::LParen),
            other => {
                if let Ok(num) = other.parse() {
                    return Ok(Token::Number(num));
                } else {
                    return Err(MathError::PlaceholderError);
                }
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

        assert_eq!(Token::Add, strings_to_parse[0].parse().unwrap());
        assert_eq!(Token::Sub, strings_to_parse[1].parse().unwrap());
        assert_eq!(Token::Mul, strings_to_parse[2].parse().unwrap());
        assert_eq!(Token::Mul, strings_to_parse[3].parse().unwrap());
        assert_eq!(Token::Div, strings_to_parse[4].parse().unwrap());
        assert_eq!(Token::Pow, strings_to_parse[5].parse().unwrap());
        assert_eq!(Token::Pow, strings_to_parse[6].parse().unwrap());
        assert_eq!(Token::RParen, strings_to_parse[7].parse().unwrap());
        assert_eq!(Token::LParen, strings_to_parse[8].parse().unwrap());
        assert_eq!(Token::Number(1.0), strings_to_parse[9].parse().unwrap());
        assert_eq!(Token::Number(2.0), strings_to_parse[10].parse().unwrap());
        assert_eq!(Token::Number(-15.0), strings_to_parse[11].parse().unwrap());
        assert!(strings_to_parse[12].parse::<Token>().is_err());
    }
}
