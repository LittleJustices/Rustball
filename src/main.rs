use std::fs;

use serenity::{
    prelude::*,
    // model::channel::Message,
    // async_trait,
};

mod message_handler;
use message_handler::Handler;
mod canned_responses;
use canned_responses::Can;
mod tests;

#[tokio::main]
async fn main() {
    let token = fs::read_to_string("DISCORD_TOKEN")
        .expect("Expected a token in the root folder");

    let responses = Can::new();

    let mut client = Client::builder(&token)
        .event_handler(Handler { responses })
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}