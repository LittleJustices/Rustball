use super::{
    dice_re::*,
    dice_errors::RollError,
};

pub fn genesys(in_command: &str) -> Result<String, RollError> {
    let mut out_command = String::new();

    for caps in GENESYS_TOKEN_RE.captures_iter(in_command) {
        if out_command.len() > 0 { out_command.push('&'); }
        let (kind, number) = (&caps["kind"], &caps["number"]);
        let sides = match kind {
            "b" | "s" => "6",
            "a" | "d" => "8",
            "p" | "c" => "12",
            other => return Err(RollError::TranslationError(other.into())),
        };

        out_command = format!("{}{}d{}g{}", out_command, number, sides, kind);
    }

    Ok(out_command)
}

pub fn exalted(in_command: &str) -> Result<String, RollError> {
    let (base, bonus) = in_command.split_once(';').unwrap_or((in_command, ""));

    // If none of the special flags are present, just give back the expression with default target and double numbers
    let out_command = if !EXALTED_TOKEN_RE.is_match(base) {
        format!("({})d10t[1, 1, 1, 2]{}", base, bonus)
    } else {
        let mut target = 7;
        let mut doubles = match base.contains('m') {
            true => None,
            false => Some(10),
        };
        let mut operations = String::new();

        for caps in EXALTED_TOKEN_RE.captures_iter(base) {
            target = match caps.name("target") {
                Some(m) => m.as_str().parse()?,
                None => target,
            };

            if let Some(d) = doubles {
                let new_doubles = match caps.name("double") {
                    Some(m) => m.as_str().parse()?,
                    None => d,
                };
                doubles = Some(new_doubles);
            }

            operations.push_str(caps.name("other").map_or("", |m| m.as_str()));
        }

        let mut success_map = vec![0; 10];
        success_map[target-1..].clone_from_slice(&vec![1; 11-target]);
        if let Some(d) = doubles {
            success_map[d-1..].clone_from_slice(&vec![2; 11-d]);
        }

        format!(
            "({})d10{}t{:?}{}",
            EXALTED_TOKEN_RE.replace_all(base, ""),
            operations,
            success_map,
            bonus
        )
    };

    Ok(out_command)
}

pub fn cofd(in_command: &str) -> Result<String, RollError> {
    if in_command.to_lowercase().trim() == "chance" {
        return Ok("1d10t10".to_string())
    }

    let (base, bonus) = in_command.split_once(';').unwrap_or((in_command, ""));

    // If none of the special flags are present, just give back the expression with default target and double numbers
    let out_command = if !COFD_TOKEN_RE.is_match(base) {
        format!("({})d10er10t8{}", base, bonus)
    } else {
        let target = 8;
        let mut again = match base.contains('m') {
            true => None,
            false => Some(10),
        };
        let mut operations = match base.contains('r') {
            true => format!("ro{:?}", (1..target).collect::<Vec<u8>>()),
            false => String::new(),
        };

        for caps in COFD_TOKEN_RE.captures_iter(base) {
            if let Some(a) = again {
                let new_again = match caps.name("again") {
                    Some(m) => m.as_str().parse()?,
                    None => a,
                };
                again = Some(new_again);
            }

            operations.push_str(caps.name("other").map_or("", |m| m.as_str()));
        }
        if let Some(a) = again {
            operations.push_str(&format!("er{:?}", (a..=10).collect::<Vec<u8>>()));
        }
        
        format!(
            "({})d10{}t{}{}",
            COFD_TOKEN_RE.replace_all(base, ""),
            operations,
            target,
            bonus
        )
    };

    Ok(out_command)
}

pub fn story_shaper(in_command: &str) -> Result<String, RollError> {
    let mut out_command = String::from("2d10");
    for caps in S3_TOKEN_RE.captures_iter(in_command) {
        let modifier = match caps.name("mod") {
            Some(m) => m.as_str(),
            None => "",
        };
        out_command.push_str(modifier);
    }

    Ok(out_command)
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn test_genesys() {
        let in_command = "a2p2 b2  d 2 c2 jkghjhgkuyguygytu s2";
        let out_command = genesys(in_command).unwrap();

        assert_eq!(out_command, "2d8ga&2d12gp&2d6gb&2d8gd&2d12gc&2d6gs".to_owned());
    }

    #[test]
    fn test_exalted() {
        let in_commands = vec![
            "5",
            "5+3",
            "5+3;+1",
            "5+3d9",
            "5+3m",
            "5+3{rr6}",
        ];
        let out_commands = vec![
            "(5)d10t[1, 1, 1, 2]",
            "(5+3)d10t[1, 1, 1, 2]",
            "(5+3)d10t[1, 1, 1, 2]+1",
            "(5+3)d10t[0, 0, 0, 0, 0, 0, 1, 1, 2, 2]",
            "(5+3)d10t[0, 0, 0, 0, 0, 0, 1, 1, 1, 1]",
            "(5+3)d10rr6t[0, 0, 0, 0, 0, 0, 1, 1, 1, 2]",
        ];

        for i in 0..in_commands.len() {
            assert_eq!(out_commands[i], exalted(in_commands[i]).unwrap());
        }
    }

    #[test]
    fn test_cofd() {
        let in_commands = vec![
            "5",
            "5+3",
            "5+3;+1",
            "5+3a9",
            "5+3m",
            "5+3r",
            "chance",
        ];
        let out_commands = vec![
            "(5)d10er10t8",
            "(5+3)d10er10t8",
            "(5+3)d10er10t8+1",
            "(5+3)d10er[9, 10]t8",
            "(5+3)d10t8",
            "(5+3)d10ro[1, 2, 3, 4, 5, 6, 7]er[10]t8",
            "1d10t10",
        ];

        for i in 0..in_commands.len() {
            assert_eq!(out_commands[i], cofd(in_commands[i]).unwrap());
        }
    }

    #[test]
    fn test_3s() {
        let in_commands: Vec<&str> = vec!["", "+5", "-(2+3)*4/5"];
        let out_commands: Vec<&str> = vec!["2d10", "2d10+5", "2d10-(2+3)*4/5"];

        for i in 0..in_commands.len() {
            assert_eq!(out_commands[i], story_shaper(in_commands[i]).unwrap());
        }
    }
}
