use std::fs;

use serenity::{
    async_trait,
    model::{
        channel::Message, 
        gateway::Ready,
        id::ChannelId,
    },
    prelude::*,
    // utils::MessageBuilder,
};

use super::canned_responses::Can;

pub struct Handler {
    pub responses: Can,
}

const PREFIX: char = '!';   // Prefix for messages Sixball will parse
const LOGGING: ChannelId = ChannelId(826898213889114183);

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        let content;
        match msg.content.trim().strip_prefix(PREFIX) {
            Some(s) => content = s,
            None => {
                if msg.channel_id == LOGGING {
                    println!("{} {}: {}", msg.timestamp, msg.author.name, msg.content);
                }
                return;
            }
        }
        let command = split_message(content);
        match &command[0][..] {
            // Shutdown order
            "bye" => {
                let bye = "Bye~! :heart:".to_string();

                self.send_msg(&ctx, msg.channel_id, bye).await;
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
                if let Ok(ans) = self.responses.find_in_can(&command[0]) {
                    self.call_and_response(&ctx, msg, ans).await;
                }
            }
        }
    }   
}

impl Handler {
    async fn send_msg(&self, context: &Context, chan: ChannelId, msg: String) {
        if let Err(why) = chan.say(&context.http, msg).await {
            println!("Error sending message: {:?}", why);
        }
    }

    async fn call_and_response(&self, context: &Context, call: Message, response: String) {
        let msg = format!("{} {}", call.author, response);
        if let Err(why) = call.channel_id.say(&context.http, msg).await {
            println!("Error sending message: {:?}", why);
        }
    }
}

fn split_message<'a>(message: &'a str) -> Vec<String> {
    // Create string vector to hold the content
    let mut content = Vec::new();

    // If the input is empty, just push and return the empty string
    if message == "" {
        content.push(message.to_string());
        return content;
    }

    // Then, split along spaces
    let arguments = message.split_whitespace();

    // Holding string for leftover comment text
    let mut leftover_text = "".to_string();

    // Loop over the first two chunks and push to content
    for (i, chunk) in arguments.enumerate() {
        if i < 2 {
            content.push(chunk.to_string().to_lowercase());
        } else {
            leftover_text = leftover_text + chunk;
        }
    }

    // Push leftovers to content as a single string, if not empty
    if leftover_text != "" { content.push(leftover_text) };
    
    // return content
    content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn message_split_test() {
        let input = [
            "roll 2d6",
            "ping",
            "roll apple",
            "roll 2d6 asdf",
            "roll! 2d6 asdfdsa!",
            "RoLl 2D6",
            "",
        ];
        
        assert_eq!(split_message(&input[0]), ["roll", "2d6"]);
        assert_eq!(split_message(&input[1]), ["ping"]);
        assert_eq!(split_message(&input[2]), ["roll", "apple"]);
        assert_eq!(split_message(&input[3]), ["roll", "2d6", "asdf"]);
        assert_eq!(split_message(&input[4]), ["roll!", "2d6", "asdfdsa!"]);
        assert_eq!(split_message(&input[5]), ["roll", "2d6"]);
        assert_eq!(split_message(&input[6]), [""]);
    }
}