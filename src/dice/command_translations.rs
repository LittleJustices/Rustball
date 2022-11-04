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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_genesys() {
        let in_command = "a2p2 b2  d 2 c2 jkghjhgkuyguygytu s2";
        let out_command = genesys(in_command).unwrap();

        assert_eq!(out_command, "2d8ga&2d12gp&2d6gb&2d8gd&2d12gc&2d6gs".to_owned());
    }
}
