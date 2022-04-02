use super::rpn_expression::RpnExpression;
use std::str::FromStr;

pub fn evaluate(infix_expression: &str) -> String {
    let rpn_expression = RpnExpression::from_str(infix_expression);
    format!("{:?}", rpn_expression)
}