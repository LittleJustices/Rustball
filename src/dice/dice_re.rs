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
    g[bsadpc]           # Genesys dice
    |
    \[.*?\]             # Array
";

const GENESYS_TOKEN_STRING: &str = r"(?x)
    (?P<kind>[bsadpc])  # Kinds of dice, named capture group
    \s*                 # Allow any number of spaces
    (?P<number>\d+)     # Number of dice of the preceding kind, named capture group
";

lazy_static!{
    pub static ref DICE_TOKEN_RE: Regex = Regex::new(&format!("(?x)(?P<token>{}|{})", MATH_TOKEN_STRING, DICE_TOKEN_STRING)).expect("Failed to compile dice token regex!");
    pub static ref GENESYS_TOKEN_RE: Regex = Regex::new(GENESYS_TOKEN_STRING).expect("Failed to compile genesys token regex!");
}
