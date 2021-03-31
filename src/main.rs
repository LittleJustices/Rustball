use std::fs;

use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "!ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "Pong!").await {
                println!("Error sending message: {:?}", why);
            }
        }
        if msg.content == "!pfp" {
            let sauce = fs::read_to_string("PFP_Source.txt");

            let answer = match sauce {
                Ok(s) => format!("My profile picture is sourced from: {}", s),
                Err(e) => {
                    println!("Failed to read PFP source file: {:?}", e);
                    "I'm sorry, I lost the source!".to_string()
                }
            };

            if let Err(why) = msg.channel_id.say(&ctx.http, answer).await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

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
