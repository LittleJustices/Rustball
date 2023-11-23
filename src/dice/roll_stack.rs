use crate::math::{rpn_token::RpnToken, math_errors::MathError};
use super::{
    dice_errors::RollError,
    roll_token::RollToken,
};

#[derive(Clone, Debug)]
pub struct RollStack {
    pub operations: Vec<RollToken>,
    pub final_result: RollToken,
}

impl RollStack {
    pub fn evaluate_string(infix_expression: &str) -> Result<Self, RollError> {
        let infix_tokens = RollToken::tokenize_expression(infix_expression)?;

        Self::evaluate_tokens(&infix_tokens)
    }

    pub fn evaluate_tokens(infix_tokens: &[RollToken]) -> Result<Self, RollError> {
        let postfix_tokens = RollToken::shunting_dice(&infix_tokens)?;

        let (operations, final_result) = Self::resolve_rpn(&postfix_tokens, &[])?;

        return Ok(RollStack { operations, final_result })
    }

    pub fn resolve_rpn(postfix_tokens: &[RollToken], starting_stack: &[RollToken]) -> Result<(Vec<RollToken>, RollToken), RollError> {
        let tokens = postfix_tokens.to_vec();
        let mut stack = starting_stack.to_vec();
        let mut operations = vec![];

        for token in tokens {
            match &token {
                RollToken::Math(rpn_token) => {
                    match rpn_token {
                        RpnToken::Number(_) => stack.push(token),
                        RpnToken::Operator(operator) => {
                            let right = stack.pop().ok_or(MathError::OperatorMismatchError)?;
                            let left = stack.pop().ok_or(MathError::OperatorMismatchError)?;
                            stack.push(RpnToken::Number(operator.apply(left.value()?.to_decimal()?, right.value()?.to_decimal()?)).into());
                        },
                        RpnToken::MathFn(math_fn) => {
                            let arg = stack.pop().ok_or(MathError::FnMismatchError)?;
                            stack.push(RpnToken::Number(math_fn.apply(arg.value()?.to_decimal()?)).into());
                        },
                        _ => return Err(RollError::MathError(MathError::MisplacedTokenError(rpn_token.clone()))),
                    }
                },
                RollToken::Argument(_) => stack.push(token),
                RollToken::Dice(dice) => {
                    let right = stack.pop().ok_or(MathError::OperatorMismatchError)?;
                    let left = stack.pop().ok_or(MathError::OperatorMismatchError)?;
                    let dice_resolved = dice.apply(left.argument()?, right.argument()?)?;
                    operations.push(RollToken::Dice(dice_resolved.clone()));
                    stack.push(RollToken::Dice(dice_resolved));
                },
                RollToken::Operator(operator) => {
                    let right = stack.pop().ok_or(MathError::OperatorMismatchError)?;
                    let left = stack.pop().ok_or(MathError::OperatorMismatchError)?;
                    let operator_resolved = operator.apply(left, right.argument()?)?;
                    operations.push(RollToken::Operator(operator_resolved.clone()));
                    stack.push(RollToken::Operator(operator_resolved));
                },
                RollToken::Conversion(conversion) => {
                    let token = stack.pop().ok_or(MathError::OperatorMismatchError)?;
                    let conversion_resolved = conversion.apply(token)?;
                    operations.push(RollToken::Conversion(conversion_resolved.clone()));
                    stack.push(RollToken::Conversion(conversion_resolved));
                },
                RollToken::Combination(combination) => {
                    let right = stack.pop().ok_or(MathError::OperatorMismatchError)?;
                    let left = stack.pop().ok_or(MathError::OperatorMismatchError)?;
                    let combination_resolved = combination.apply(left, right)?;
                    // Might uncomment this if hypothetical future combinations need to be displayed
                    // operations.push(RollToken::Combination(combination_resolved.clone()));
                    stack.push(RollToken::Combination(combination_resolved));
                }
            }
        }

        if stack.len() != 1 {
            Err(RollError::MathError(MathError::TrailingTokensError))
        } else {
            let final_result = stack.last().ok_or(MathError::ImpossibleError)?.clone();
            Ok((operations, final_result))
        }
    }

    pub fn append_from_string(&mut self, infix_expression: &str) -> Result<(), RollError> {
        let infix_tokens = RollToken::tokenize_expression(infix_expression)?;
        let postfix_tokens = RollToken::shunting_dice(&infix_tokens)?;

        let previous_result = self.final_result.clone();
        let (mut new_operations, new_result) = Self::resolve_rpn(&postfix_tokens, &[previous_result])?;

        self.operations.append(&mut new_operations);
        self.final_result = new_result;

        Ok(())
    }
}
