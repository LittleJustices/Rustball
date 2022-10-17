use super::{
    dice_errors::RollError,
    roll_stack::RollStack,
    result_kinds::*,
};

pub enum RollResult {
    SumDice(SumDice),
}
