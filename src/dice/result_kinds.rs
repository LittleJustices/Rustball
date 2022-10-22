use super::{
    dice_errors::RollError,
    roll_stack::RollStack,
};

#[derive(Debug)]
pub struct NumericResult {
    pub result: f64,
}

impl NumericResult {
    pub fn evaluate(roll_stack: &RollStack) -> Result<Self, RollError> {
        let result = roll_stack.final_result.value()?;

        Ok(NumericResult { result })
    }
}
