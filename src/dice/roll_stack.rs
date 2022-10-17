use crate::math::rpn_token::RpnToken;
use super::{
    dice_errors::RollError,
    roll_token::RollToken,
};

#[derive(Debug)]
pub struct RollStack {
    pub operations: Vec<RollToken>,
    pub stack: Vec<RollToken>,
    pub final_result: RollToken,
}

impl RollStack {
    pub fn evaluate_string(infix_expression: &str) -> Result<Self, RollError> {
        let infix_tokens = RollToken::tokenize_expression(infix_expression)?;

        Self::evaluate_tokens(&infix_tokens)
    }

    pub fn evaluate_tokens(infix_tokens: &[RollToken]) -> Result<Self, RollError> {
        let postfix_tokens = RollToken::shunting_dice(&infix_tokens)?;

        Self::resolve_rpn(&postfix_tokens)
    }

    pub fn resolve_rpn(postfix_tokens: &[RollToken]) -> Result<Self, RollError> {
        let tokens = postfix_tokens.to_vec();
        let mut stack = vec![];
        let mut operations = vec![];

        for token in tokens {
            match &token {
                RollToken::Math(rpn_token) => {
                    match rpn_token {
                        RpnToken::Number(_) => stack.push(token),
                        RpnToken::Operator(operator) => {
                            let right = stack.pop().ok_or(RollError::PlaceholderError)?;
                            let left = stack.pop().ok_or(RollError::PlaceholderError)?;
                            stack.push(RpnToken::Number(operator.apply(left.value()?, right.value()?)).into());
                        },
                        RpnToken::MathFn(math_fn) => {
                            let arg = stack.pop().ok_or(RollError::PlaceholderError)?;
                            stack.push(RpnToken::Number(math_fn.apply(arg.value()?)).into());
                        },
                        _ => return Err(RollError::PlaceholderError),
                    }
                },
                RollToken::Argument(_) => stack.push(token),
                RollToken::Dice(dice) => {
                    let right = stack.pop().ok_or(RollError::PlaceholderError)?;
                    let left = stack.pop().ok_or(RollError::PlaceholderError)?;
                    let dice_resolved = dice.apply(left.argument()?, right.argument()?)?;
                    operations.push(RollToken::Dice(dice_resolved.clone()));
                    stack.push(RollToken::Dice(dice_resolved));
                },
                RollToken::Operator(operator) => {
                    let right = stack.pop().ok_or(RollError::PlaceholderError)?;
                    let left = stack.pop().ok_or(RollError::PlaceholderError)?;
                    let operator_resolved = operator.apply(left.pool()?, right.argument()?)?;
                    operations.push(RollToken::Operator(operator_resolved.clone()));
                    stack.push(RollToken::Operator(operator_resolved));
                },
                RollToken::Conversion(conversion) => {
                    let right = stack.pop().ok_or(RollError::PlaceholderError)?;
                    let left = stack.pop().ok_or(RollError::PlaceholderError)?;
                    let conversion_resolved = conversion.apply(left, right.argument()?)?;
                    operations.push(RollToken::Conversion(conversion_resolved.clone()));
                    stack.push(RollToken::Conversion(conversion_resolved));
                },
            }
        }

        if stack.len() != 1 {
            Err(RollError::PlaceholderError)
        } else {
            let final_result = stack.last().ok_or(RollError::PlaceholderError)?.clone();
            Ok(RollStack { operations, stack, final_result })
        }
    }
}
