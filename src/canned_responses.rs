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

        Can { responses }
    }
    pub fn find_in_can(&self, call: &String) -> Result<String, ErrorKind> {
        match self.responses.get(call) {
            Some(response) => Ok(response.to_string()),
            None => Err(ErrorKind::NotFound)
        }
    }
}