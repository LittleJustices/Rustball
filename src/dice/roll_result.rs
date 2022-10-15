use super::roll_token::RollToken;

pub trait RollResult {
    fn evaluate(operations: &[RollToken]) -> Self;

    fn summarize(&self) -> String;
}
