use super::{
    dice_errors::RollError,
    roll_stack::RollStack,
    result_kinds::*,
};

pub enum RollResult {
    Numeric(NumericResult),
}

impl RollResult {
    pub fn evaluate(roll_stack: &RollStack) -> Result<Self, RollError> {
        Ok(RollResult::Numeric(NumericResult::evaluate(roll_stack)?))
    }
}
