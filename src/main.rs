use std::fs;

use serenity::{
    prelude::*,
};

mod messaging;
use messaging::{
    message_handler::Handler,
};

#[tokio::main]
async fn main() {
    let token = fs::read_to_string("DISCORD_TOKEN")
        .expect("Expected a token in the root folder");

    let mut client = Client::builder(&token)
        .event_handler(Handler::new())
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}