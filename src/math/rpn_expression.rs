use std::{
    collections::HashMap,
    str::FromStr,
};
use lazy_static::lazy_static;
use super::math_errors::MathParseError;

lazy_static! { 
    static ref PRECEDENCE: HashMap<char, u8> = HashMap::from([
        ('^', 6),
        ('%', 6),
        ('*', 5),
        ('x', 5),
        ('/', 5),
        ('+', 4),
        ('-', 4),
        ('(', 0),
        (')', 0),
        ]);
}

#[derive(Debug)]
pub struct RpnExpression {
    infix_expression: String,
}

impl FromStr for RpnExpression {
    type Err = MathParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(RpnExpression { infix_expression: s.to_owned() })
    }
}