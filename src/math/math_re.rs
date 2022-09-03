use lazy_static::lazy_static;
use regex::Regex;

const MATH_TOKEN_STRING: &str = r"(?x)
    (?P<token>
        [\^%\*x/\+\-\(\)]   # Math operators
        |
        \d+\.?\d*           # Numbers
        |
        pi|π                # Pi
        |
        a?(?:sin|cos|tan)h? # Trig functions
        |
        sqrt                # Square root
        |
        abs                 # Absolute value
    )";

lazy_static!{
    pub static ref MATH_TOKEN_RE: Regex = Regex::new(MATH_TOKEN_STRING).expect("Failed to compile math token regex!");
}
