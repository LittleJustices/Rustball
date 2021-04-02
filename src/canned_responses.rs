use std::collections::HashMap;
use std::io::ErrorKind;

pub struct Can {
    responses: HashMap<String, String>
}

impl Can {
    pub fn new() -> Can {
        let mut responses = HashMap::new();

        responses.insert(String::from("!ping"), String::from("Pong!"));
        responses.insert(String::from("!squid"), String::from("＜コ:彡"));
        responses.insert(
            String::from("!shadowruns"), 
            String::from(
                "Say it with me now:\n\t\t\t\tWatch your back.\n\t\t\t\tShoot straight.\n\t\t\t\tConserve ammo.\n\t\t\t\tAnd never, ever deal with a dragon!\n(ﾉ≧∀≦)ﾉ"
            )
        );
        responses.insert(String::from("!roll"), String::from("I don't know how to roll dice yet!"));

        Can { responses }
    }
    
    pub fn find_in_can(&self, call: &String) -> Result<String, ErrorKind> {
        match self.responses.get(call) {
            Some(response) => Ok(response.to_string()),
            None => Err(ErrorKind::NotFound)
        }
    }
}