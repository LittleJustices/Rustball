use super::{
    dice_errors::RollError,
};
use std::fmt;

#[derive(Debug)]
pub enum RollValue {
    Decimal(f64),
    Successes(i16),
}

impl RollValue {
    pub fn to_decimal(self) -> Result<f64, RollError> {
        match self {
            RollValue::Decimal(number) => Ok(number),
            RollValue::Successes(sux) => Ok(sux as f64),
        }
    }

    pub fn add(self, other: RollValue) -> Result<Self, RollError> {
        match self {
            RollValue::Decimal(left) => match other {
                RollValue::Decimal(right) => Ok(RollValue::Decimal(left + right)),
                RollValue::Successes(right) => Ok(RollValue::Decimal(left + (right as f64))),
            },
            RollValue::Successes(left) => match other {
                RollValue::Decimal(right) => Ok(RollValue::Decimal((left as f64) + right)),
                RollValue::Successes(right) => Ok(RollValue::Successes(left + right)),
            },
        }
    }
}

impl fmt::Display for RollValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RollValue::Decimal(number) => write!(f, "{}", number),
            RollValue::Successes(sux) => write!(f, "{}", sux),
        }
    }
}
