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

const PREFIX: char = '!';   // Prefix for messages Sixball will parse

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let content;
        match msg.content.strip_prefix(PREFIX) {
            Some(s) => content = s,
            None => return
        }
        match content {
            // Shutdown order
            "bye" => {
                let bye = "Bye~! :heart:".to_string();

                self.call_and_response(&ctx, msg, bye).await;
                std::process::exit(0);
            },
            // Source for profile pic
            "pfp" => {
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
            // Any other order, check if it's a canned response, otherwise do nothing
            _ => {
                // If find_in_can returns a result (not error), send the response to channel, otherwise ignore
                if let Ok(ans) = self.responses.find_in_can(content) {
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

fn split_message<'a>(message: &'a String) -> Vec<&'a str> {
    // Create string vector to hold the content
    let mut content = Vec::<&'a str>::new();

    // If message does not have the prefix, just return the body of the message
    if !message.starts_with(PREFIX) { 
        content.push(message);
        return content;
    }

    // If it does, remove the prefix (first char in string)
    let mut chars = message.chars();
    chars.next();
    let command = chars.as_str();

    // If the string is now empty, just push and return an empty string
    if command == "" {
        content.push("");
        return content;
    }

    // Then, split along spaces
    let mut arguments = command.split_whitespace();

    // Loop over the chunks
    for chunk in arguments {
        // First iteration, we want to push an empty string and return if it's None and push the chunk if Some
        // Second iteration, we want to push the chunk if it's Some and 
    }
    
    // push the first element
    // If there is a second, push that too
    // If there are more left, concatenate them all into one string and push that
    content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_split_test() {
        let input = [
            String::from("!roll 2d6"),
            String::from("!ping"),
            String::from("!roll apple"),
            String::from("!roll 2d6 asdf"),
            String::from("&roll 2d6 asdf"),
            String::from("roll! 2d6 asdfdsa!"),
            String::from("!RoLl 2D6"),
            String::from("!"),
            String::from("Lorem Ipsum"),
            String::from(""),
        ];
        
        assert_eq!(split_message(&input[0]), ["roll", "2d6"]);
        assert_eq!(split_message(&input[8]), ["Lorem Ipsum"]);
    }
}