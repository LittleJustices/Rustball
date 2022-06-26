use crate::math::rpn_token::RpnToken;
use super::die::Die;

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum RollToken {
    Math(RpnToken),
    Die(Die),
    Explode(Explode),
    Keep(Keep),
    Reroll(Reroll),
    Target(Target),
    Botch(Target),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Explode {
    Single(Vec<u8>),
    Recursive(Vec<u8>),
}

#[derive(Debug, PartialEq)]
pub enum Keep {
    Low(u8),
    High(u8),
    All,
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Reroll {
    Single(Vec<u8>),
    Recursive(Vec<u8>),
    Additive(Vec<u8>),
}

#[allow(dead_code)]
#[derive(Debug, PartialEq)]
pub enum Target {
    Single(u8),
    Complex(Vec<u8>)
}
