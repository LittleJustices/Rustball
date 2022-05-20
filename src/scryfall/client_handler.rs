use reqwest::Client;
use chrono::prelude::*;

pub struct ClientHandler {
    last_request: DateTime<Utc>,
    client: Client
}

impl ClientHandler {
    pub fn new() -> ClientHandler {
        let last_request = Utc::now();
        let client = Client::new();

        ClientHandler { last_request, client }
    }
}
