use lazy_static::lazy_static;
use regex::Regex;

const MATH_TOKEN_STRING: &str = r"(?P<token>[\^%\*x/\+\-\(\)]|\d+\.?\d*)";

lazy_static!{
    pub static ref MATH_TOKEN_RE: Regex = Regex::new(MATH_TOKEN_STRING).expect("Failed to compile math token regex!");
}
