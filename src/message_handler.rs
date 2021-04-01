use std::fs;

use serenity::{
    async_trait,
    model::{
        channel::Message, 
        gateway::Ready
    },
    prelude::*,
    // utils::MessageBuilder,
};

use super::canned_responses::Can;

pub struct Handler {
    pub responses: Can,
}

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        match &msg.content[..] {
            "!bye" => {
                let bye = "Bye~! :heart:".to_string();

                self.call_and_response(&ctx, msg, bye).await;
                std::process::exit(0);
            },
            "!pfp" => {
                let sauce = fs::read_to_string("PFP_Source.txt");

                let answer = match sauce {
                    Ok(s) => format!("My profile picture is sourced from: {}", s),
                    Err(e) => {
                        println!("Failed to read PFP source file: {:?}", e);
                        ":radioactive: I'm sorry, I lost the source! :radioactive:".to_string()
                    }
                };

                self.call_and_response(&ctx, msg, answer).await;
            },
            _ => {
                // If find_in_can returns a result (not error), send the response to channel, otherwise ignore
                if let Ok(ans) = self.responses.find_in_can(&msg.content) {
                    self.call_and_response(&ctx, msg, ans).await;
                }
            }
        }
    }   
}

impl Handler {
    async fn call_and_response(&self, context: &Context, call: Message, response: String) {
        if let Err(why) = call.channel_id.say(&context.http, response).await {
            println!("Error sending message: {:?}", why);
        }
    }
}