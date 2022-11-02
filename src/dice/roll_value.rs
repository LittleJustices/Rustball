use super::{
    dice_errors::RollError,
    value_kinds::*,
};
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum RollValue {
    Decimal(f64),
    Successes(i16),
    Genesys(GenesysValue),
}

impl RollValue {
    pub fn to_decimal(self) -> Result<f64, RollError> {
        match self {
            RollValue::Decimal(number) => Ok(number),
            RollValue::Successes(sux) => Ok(sux as f64),
            _ => Err(RollError::NotANumberError),
        }
    }

    pub fn add(self, other: RollValue) -> Result<Self, RollError> {
        match self {
            RollValue::Decimal(left) => match other {
                RollValue::Decimal(right) => Ok(RollValue::Decimal(left + right)),
                RollValue::Successes(right) => Ok(RollValue::Decimal(left + (right as f64))),
                _ => Err(RollError::NotANumberError),
            },
            RollValue::Successes(left) => match other {
                RollValue::Decimal(right) => Ok(RollValue::Decimal((left as f64) + right)),
                RollValue::Successes(right) => Ok(RollValue::Successes(left + right)),
                _ => Err(RollError::NotANumberError),
            },
            RollValue::Genesys(left) => match other {
                RollValue::Genesys(right) => Ok(RollValue::Genesys(left.add(right))),
                _ => Err(RollError::NotANumberError),
            },
        }
    }
}

impl fmt::Display for RollValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RollValue::Decimal(number) => write!(f, "{}", number),
            RollValue::Successes(sux) => write!(f, "{}", sux),
            RollValue::Genesys(gen_val) => write!(f, "{}", gen_val),
        }
    }
}

impl<T> From<T> for RollValue
    where T: Into<f64>
{
    fn from(number: T) -> Self {
        RollValue::Decimal(number.into())
    }
}
