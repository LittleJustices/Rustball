use reqwest::{Client, ClientBuilder};
use chrono::{prelude::*, Duration};

const COOLDOWN: i64 = 100;  // Cooldown between requests in ms
static APP_USER_AGENT: &str = concat!(
    env!("CARGO_PKG_NAME"),
    "/",
    env!("CARGO_PKG_VERSION"),
);

pub struct ClientHandler {
    last_request: DateTime<Utc>,
    cooldown: Duration,
    client: Client
}

impl ClientHandler {
    pub fn new() -> ClientHandler {
        let last_request = Utc::now();
        let cooldown = Duration::milliseconds(COOLDOWN);
        let client_builder = ClientBuilder::new()
            .user_agent(APP_USER_AGENT);
        let client = client_builder.build().expect("Failed to build client from builder!");

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
