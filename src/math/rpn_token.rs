use std::str::FromStr;
use super::math_errors::MathError;

#[derive(Clone, Debug, PartialEq)]
pub enum RpnToken {
    Number(f64),
    Operator(Operator),
    MathFn(MathFn),
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
            ")" | "]" | "}" => Ok(RpnToken::RParen),
            "(" | "[" | "{" => Ok(RpnToken::LParen),
            other => {
                if let Ok(number) = other.parse() {
                    Ok(RpnToken::Number(number))
                } else if let Ok(operator) = other.parse() {
                    Ok(RpnToken::Operator(operator))
                } else if let Ok(math_fn) = other.parse() {
                    Ok(RpnToken::MathFn(math_fn))
                } else {
                    Err(MathError::SymbolError(s.into()))
                }
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
    Mod,
}

impl Operator {
    pub fn precedence(&self) -> u8 {
        match self {
            Operator::Add | Operator::Sub => 4,
            Operator::Mul | Operator::Div | Operator::Mod => 5,
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
            Operator::Mod => left % right,
            Operator::Pow => left.powf(right),
        }
    }
}

impl FromStr for Operator {
    type Err = MathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "+" => Ok(Operator::Add),
            "-" => Ok(Operator::Sub),
            "*" | "x" => Ok(Operator::Mul),
            "/" => Ok(Operator::Div),
            "%" => Ok(Operator::Mod),
            "^" | "**" => Ok(Operator::Pow),
            _ => Err(MathError::PlaceholderError),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum MathFn {
    Sin,
    Asin,
    Sinh,
    Asinh,
    Cos,
    Acos,
    Cosh,
    Acosh,
    Tan,
    Atan,
    Tanh,
    Atanh,
    Sqrt,
}

impl MathFn {
    pub fn apply(&self, arg: f64) -> f64 {
        match self {
            MathFn::Sin => arg.sin(),
            MathFn::Cos => arg.cos(),
            MathFn::Tan => arg.tan(),
            MathFn::Asin => arg.asin(),
            MathFn::Sinh => arg.sinh(),
            MathFn::Acos => arg.acos(),
            MathFn::Cosh => arg.cosh(),
            MathFn::Atan => arg.atan(),
            MathFn::Tanh => arg.tanh(),
            MathFn::Asinh => arg.asinh(),
            MathFn::Acosh => arg.acosh(),
            MathFn::Atanh => arg.atanh(),
            MathFn::Sqrt => arg.sqrt(),
        }
    }
}

impl FromStr for MathFn {
    type Err = MathError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.trim() {
            "sin" => Ok(MathFn::Sin),
            "asin" => Ok(MathFn::Asin),
            "sinh" => Ok(MathFn::Sinh),
            "asinh" => Ok(MathFn::Asinh),
            "cos" => Ok(MathFn::Cos),
            "acos" => Ok(MathFn::Acos),
            "cosh" => Ok(MathFn::Cosh),
            "acosh" => Ok(MathFn::Acosh),
            "tan" => Ok(MathFn::Tan),
            "atan" => Ok(MathFn::Atan),
            "tanh" => Ok(MathFn::Tanh),
            "atanh" => Ok(MathFn::Atanh),
            "sqrt" => Ok(MathFn::Sqrt),
            _ => Err(MathError::PlaceholderError),
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
