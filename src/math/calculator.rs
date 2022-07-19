use crate::sixball_errors::SixballError;

use super::{
    math_errors::MathError,
    rpn_token::RpnToken,
};
use super::rpn_expression::RpnExpression;
use std::str::FromStr;

pub fn evaluate(infix_expression: &str) -> Result<f64, SixballError> {
    let rpn_expression = RpnExpression::from_str(infix_expression)?;
    Ok(resolve_rpn(rpn_expression)?)
}

pub fn resolve(postfix_expression: &[RpnToken]) -> Result<f64, MathError> {
    let tokens = postfix_expression.to_vec();
    let mut stack = vec![];

    for token in tokens {
        match token {
            RpnToken::Number(number) => stack.push(number),
            other => {
                match other {
                    RpnToken::Add => {
                        let right = stack.pop().ok_or(MathError::PlaceholderError)?;
                        let left = stack.pop().ok_or(MathError::PlaceholderError)?;
                        stack.push(left + right);
                    },
                    RpnToken::Sub => {
                        let right = stack.pop().ok_or(MathError::PlaceholderError)?;
                        let left = stack.pop().ok_or(MathError::PlaceholderError)?;
                        stack.push(left - right);
                    },
                    RpnToken::Mul => {
                        let right = stack.pop().ok_or(MathError::PlaceholderError)?;
                        let left = stack.pop().ok_or(MathError::PlaceholderError)?;
                        stack.push(left * right);
                    },
                    RpnToken::Div => {
                        let right = stack.pop().ok_or(MathError::PlaceholderError)?;
                        let left = stack.pop().ok_or(MathError::PlaceholderError)?;
                        stack.push(left / right);
                    },
                    RpnToken::Pow => {
                        let right = stack.pop().ok_or(MathError::PlaceholderError)?;
                        let left = stack.pop().ok_or(MathError::PlaceholderError)?;
                        stack.push(left.powf(right));
                    },
                    _ => return Err(MathError::PlaceholderError)
                }
            }
        }
    }

    if stack.len() != 1 {
        Err(MathError::PlaceholderError)
    } else {
        stack.pop().ok_or(MathError::PlaceholderError)
    }
}

fn resolve_rpn(rpn_expression: RpnExpression) -> Result<f64, MathError> {
    let mut queue = rpn_expression.get_rpn_expression().clone();
    let mut stack: Vec<f64> = vec![];

    while queue.len() > 0 {
        if let Some(item) = queue.pop_front() {
            if let Ok(number) = item.parse::<f64>() {
                stack.push(number);
            } else {
                if stack.len() >= 2 {
                    let right = stack.pop().unwrap_or_default();
                    let left = stack.pop().unwrap_or_default();
                    let value;

                    match item.as_str() {
                        "*" => { value = left * right },
                        "x" => { value = left * right },
                        "/" => { value = left / right },
                        "+" => { value = left + right },
                        "-" => { value = left - right },
                        "%" => { value = left % right },
                        "^" => { value = left.powf(right) },
                        _ => { return Err(MathError::SymbolError(item)) /* Operator not found error */ }
                    }
                    stack.push(value);
                }
                else {
                    return Err(MathError::ExpressionError("I'm running out of operands here!".to_owned())) /* not enough operands error */
                }
            }
        }
    }

    if stack.len() != 1 {
        return Err(MathError::ExpressionError("I don't think the operands and operators match up right...".to_owned())) /* Mismatched operators and operands error */
    } else {
        return Ok(stack.pop().unwrap_or_default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rpn() {
        let expression = "3+4*2/(1-5)^2^3";
        let result = 3.0001220703125;
        let token_vector = RpnExpression::tokenize_expression(expression).unwrap();
        let postfix_expression = RpnExpression::shunting_yard(&token_vector).unwrap();

        assert_eq!(resolve(&postfix_expression).unwrap(), result);
    }
}
