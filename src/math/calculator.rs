use crate::sixball_errors::SixballError;

use super::{
    math_errors::MathError,
    rpn_token::RpnToken,
};
use super::rpn_expression::RpnExpression;

pub fn evaluate_string(infix_expression: &str) -> Result<f64, SixballError> {
    let infix_tokens = RpnExpression::tokenize_expression(infix_expression)?;
    
    Ok(evaluate_tokens(&infix_tokens)?)
}

pub fn evaluate_tokens(infix_tokens: &[RpnToken]) -> Result<f64, MathError> {
    let postfix_tokens = RpnExpression::shunting_yard(infix_tokens)?;
    resolve_rpn(&postfix_tokens, &[])
}

pub fn resolve_rpn(postfix_expression: &[RpnToken], starting_stack: &[f64]) -> Result<f64, MathError> {
    let tokens = postfix_expression.to_vec();
    let mut stack = starting_stack.to_vec();

    for token in tokens {
        match token {
            RpnToken::Number(number) => stack.push(number),
            RpnToken::Operator(operator) => {
                let right = stack.pop().ok_or(MathError::OperatorMismatchError)?;
                let left = stack.pop().ok_or(MathError::OperatorMismatchError)?;
                stack.push(operator.apply(left, right));
            },
            RpnToken::MathFn(math_fn) => {
                let arg = stack.pop().ok_or(MathError::FnMismatchError)?;
                stack.push(math_fn.apply(arg));
            },
            _ => return Err(MathError::MisplacedTokenError(token))
        }
    }

    if stack.len() != 1 {
        Err(MathError::TrailingTokensError)
    } else {
        stack.pop().ok_or(MathError::ImpossibleError)
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

        assert_eq!(resolve_rpn(&postfix_expression, &[]).unwrap(), result);
    }
}
