// use super::{
//     dice_errors::RollError,
//     roll_result::RollResult,
//     roll_stack::RollStack,
// };

#[derive(Debug)]
pub struct SumDice {
    pub result: f64,
}

impl SumDice {
    fn new() -> Self {
        SumDice { result: 0.0 }
    }
}
