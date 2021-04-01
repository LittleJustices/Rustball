use std::fs;

use serenity::{
    prelude::*,
    // model::channel::Message,
    // async_trait,
};

mod message_handler;
use message_handler::Handler;

#[tokio::main]
async fn main() {
    let token = fs::read_to_string("DISCORD_TOKEN")
        .expect("Expected a token in the root folder");

    let mut client = Client::builder(&token)
        .event_handler(Handler)
        .await
        .expect("Error creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

// #[async_trait]
// async fn send_message(ctx: Context, call: Message, response: String) {
//     if let Err(why) = call.channel_id.say(&ctx.http, response).await {
        
//     }
// }