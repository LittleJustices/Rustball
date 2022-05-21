use reqwest::Client;
use chrono::{prelude::*, Duration};

const COOLDOWN: i64 = 100;  // Cooldown between requests in ms

pub struct ClientHandler {
    last_request: DateTime<Utc>,
    cooldown: Duration,
    client: Client
}

impl ClientHandler {
    pub fn new() -> ClientHandler {
        let last_request = Utc::now();
        let cooldown = Duration::milliseconds(COOLDOWN);
        let client = Client::new();

        ClientHandler { last_request, cooldown, client }
    }

    pub fn client(&mut self) -> &Client {
        self.last_request = Utc::now();
        &self.client
    }

    pub fn client_available(&self) -> bool {
        Utc::now().signed_duration_since(self.last_request) > self.cooldown
    }
}
