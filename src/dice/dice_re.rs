use lazy_static::lazy_static;
use regex::Regex;
use crate::math::math_re::MATH_TOKEN_STRING;

const DICE_TOKEN_STRING: &str = r"
    d                   # Dice notation
    |
    [tb]                # Target number or botch number
    |
    k[elh]?             # Keep
    |
    r[obwr]?            # Reroll
    |
    e[aor]?             # Explode
    |
    \[.*?\]             # Array
";

lazy_static!{
    pub static ref DICE_TOKEN_RE: Regex = Regex::new(&format!("(?x)(?P<token>{}|{})", MATH_TOKEN_STRING, DICE_TOKEN_STRING)).expect("Failed to compile dice token regex!");
}
