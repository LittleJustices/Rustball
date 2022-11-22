use serde::{Deserialize, Serialize};
use serde_json;
use std::fs;

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub discord_token: String,
    pub prefix: String,
    pub comment_separator: String,
    pub repeater_separator: String,
    pub log_folder_path: String,
    pub pfp_source: String,
}

impl Config {
    pub fn new() -> Config {
        let data = fs::read_to_string("config.json").expect("Failed to find config.json file");

        let config = serde_json::from_str(&data).expect("Error parsing config data");

        config
    }
}