use super::{
    dice_errors::RollError,
    roll_stack::RollStack,
};

#[derive(Debug)]
pub struct DecimalValue {
    pub result: f64,
}

impl DecimalValue {
    pub fn evaluate(roll_stack: &RollStack) -> Result<Self, RollError> {
        let result = roll_stack.final_result.value()?;

        Ok(DecimalValue { result })
    }
}
