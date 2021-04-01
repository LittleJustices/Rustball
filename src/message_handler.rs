use std::fs;

use serenity::{
    async_trait,
    model::{
        channel::Message, 
        gateway::Ready
    },
    prelude::*,
    utils::MessageBuilder,
};

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        match &msg.content[..] {
            "!ping" => {
                let pong = MessageBuilder::new()
                    .push("@")
                    .push(&msg.author.name)
                    .push(" Pong!")
                    .build();

                self.call_and_response(&ctx, msg, pong).await;
                },
            "!pfp" => {
                let sauce = fs::read_to_string("PFP_Source.txt");

                let answer = match sauce {
                    Ok(s) => format!("My profile picture is sourced from: {}", s),
                    Err(e) => {
                        println!("Failed to read PFP source file: {:?}", e);
                        "I'm sorry, I lost the source!".to_string()
                    }
                };

                self.call_and_response(&ctx, msg, answer).await;
            },
            "!bye" => {
                let bye = "Bye~! :heart:".to_string();

                self.call_and_response(&ctx, msg, bye).await;
                std::process::exit(0);
            }
            _ => {}
        }
    }   

    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

impl Handler {
    async fn call_and_response(&self, context: &Context, call: Message, response: String) {
        if let Err(why) = call.channel_id.say(&context.http, response).await {
            println!("Error sending message: {:?}", why);
        }
    }
}