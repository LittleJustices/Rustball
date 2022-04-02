use super::math_errors::MathError;
use super::rpn_expression::RpnExpression;
use std::str::FromStr;

pub fn evaluate(infix_expression: &str) -> Result<String, MathError> {
    let rpn_expression = RpnExpression::from_str(infix_expression)?;
    let result = match resolve_rpn(rpn_expression) {
        Ok(res) => Ok(format!("{} = {}", infix_expression, res)),
        Err(why) => Err(why)
    };
    result
}

fn resolve_rpn(rpn_expression: RpnExpression) -> Result<f64, MathError> {
    let mut queue = rpn_expression.get_rpn_expression().clone();
    let mut stack: Vec<f64> = vec![];

    while queue.len() > 0 {
        if let Some(item) = queue.pop_front() {
            if let Ok(number) = item.parse::<f64>() {
                stack.push(number);
            } else {
                if stack.len() > 2 {
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