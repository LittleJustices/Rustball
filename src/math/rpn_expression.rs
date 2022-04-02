use std::str::FromStr;
use super::math_errors::MathParseError;

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