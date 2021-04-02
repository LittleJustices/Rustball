use std::collections::HashMap;
use std::io::ErrorKind;

pub struct Can {
    responses: HashMap<String, String>
}

impl Can {
    pub fn new() -> Can {
        let mut responses = HashMap::new();

        responses.insert(
            String::from("ping"), 
            String::from("Pong!")
        );
        responses.insert(
            String::from("squid"), 
            String::from("＜コ:彡")
        );
        responses.insert(
            String::from("shadowruns"), 
            String::from(
                "Say it with me now:
                \t\t\t\tWatch your back.
                \t\t\t\tShoot straight.
                \t\t\t\tConserve ammo.
                \t\t\t\tAnd never, ever deal with a dragon!
                (ﾉ≧∀≦)ﾉ"
            )
        );
        responses.insert(
            String::from("unyu"), 
            String::from("うにゅうー！")
        );
        responses.insert(
            String::from("help"), 
            String::from("If you read this, please tell my boss to write the documentation already!")
        );

        // To be removed when !roll works
        responses.insert(String::from("roll"), String::from("I don't know how to roll dice yet!"));
        responses.insert(String::from("wod"), String::from("I'm not edgy enough for that yet!"));
        responses.insert(String::from("cod"), String::from("I'm not edgy enough for that yet!"));
        responses.insert(String::from("cofd"), String::from("I'm not edgy enough for that yet!"));
        responses.insert(String::from("l5r"), String::from("I'm not weeb enough for that yet!"));
        responses.insert(String::from("rings"), String::from("I'm not weeb enough for that yet!"));

        Can { responses }
    }

    pub fn find_in_can(&self, call: &str) -> Result<String, ErrorKind> {
        match self.responses.get(call) {
            Some(response) => Ok(response.to_string()),
            None => Err(ErrorKind::NotFound)
        }
    }
}