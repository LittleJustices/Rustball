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
    &                   # Merge
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

const EXALTED_TOKEN_STRING: &str = r"(?x)
    m                   # No doubles (m for mortal, from 2e)
    |
    s(?P<target>\d+)    # Target number (s for sidereal)
    |
    d(?P<double>\d+)    # Doubles number
    |
    \{(?P<other>.*)\}   # Dice operations
";

const COFD_TOKEN_STRING: &str = r"(?x)
r                   # Rote
|
m                   # No N-again (m for mortal, from Ex2)
|
a(?P<again>\d+)     # N-again number
";

const S3_TOKEN_STRING: &str = r"(?x)
    (?P<mod>[+-]\S+)    # Modifier or penalty
";

lazy_static!{
    pub static ref DICE_TOKEN_RE: Regex = Regex::new(&format!("(?x)(?P<token>{}|{})", MATH_TOKEN_STRING, DICE_TOKEN_STRING)).expect("Failed to compile dice token regex!");
    pub static ref GENESYS_TOKEN_RE: Regex = Regex::new(GENESYS_TOKEN_STRING).expect("Failed to compile genesys token regex!");
    pub static ref EXALTED_TOKEN_RE: Regex = Regex::new(EXALTED_TOKEN_STRING).expect("Failed to compile exalted token regex!");
    pub static ref COFD_TOKEN_RE: Regex = Regex::new(COFD_TOKEN_STRING).expect("Failed to compile cofd token regex!");
    pub static ref S3_TOKEN_RE: Regex = Regex::new(S3_TOKEN_STRING).expect("Failed to compile story shaper token regex!");
}
