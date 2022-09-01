use lazy_static::lazy_static;
use regex::Regex;

const DICE_MATCH_STRING: &str = r"(?P<number>\d+)d(?P<sides>\d+)";
const DICE_TOKEN_STRING: &str = r"(?x)
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
    )";

lazy_static!{
    pub static ref DICE_MATCH_RE: Regex = Regex::new(DICE_MATCH_STRING).expect("Failed to compile dice expression regex!");
    pub static ref DICE_TOKEN_RE: Regex = Regex::new(DICE_TOKEN_STRING).expect("Failed to compile dice token regex!");
}