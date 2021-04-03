use std::io::ErrorKind;
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

use super::{
    canned_responses::Can,
    logger,
};

pub struct Handler {
    pub responses: Can,
}

const PREFIX: char = '!';   // Prefix for messages Sixball will parse
// const LOGGING: ChannelId = ChannelId(826898213889114183);

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
                if logger::logging().contains(&msg.channel_id.0) {
                    println!("{} {}: {}", msg.timestamp, msg.author.name, msg.content);
                }
                return;
            }
        }
        let command = split_message(content);
        match &command[0][..] {
            // Shutdown order
            "bye" => {
                let bye = "Bye~! ❤".to_string();

                self.send_msg(&ctx, msg.channel_id, bye).await;
                std::process::exit(0);
            },
            "log" => {
                match logger::log_channel(msg.channel_id.0) {
                    Ok(c) => {
                        let log_confirm = format!("Logging <#{}> now! ❤", c);
                        self.send_msg(&ctx, msg.channel_id, log_confirm).await;
                    },
                    Err(_) => {
                        let log_error = "☢ I'm already logging that channel! ☢".to_string();
                        self.call_and_response(&ctx, msg, log_error).await;
                    }
                }
            },
            "unlog" => {
                match logger::unlog_channel(msg.channel_id.0) {
                    Ok(c) => {
                        let log_confirm = format!("Okay, I'll stop logging <#{}>! ❤", c);
                        self.send_msg(&ctx, msg.channel_id, log_confirm).await;
                    },
                    Err(_) => {
                        let log_error = "☢ I'm not logging that channel yet! ☢".to_string();
                        self.call_and_response(&ctx, msg, log_error).await;
                    }
                }
            },
            // Source for profile pic
            "pfp" => {
                let sauce = fs::read_to_string("PFP_Source.txt");

                let answer = match sauce {
                    Ok(s) => format!("My profile picture is sourced from: {}", s),
                    Err(e) => {
                        println!("Failed to read PFP source file: {:?}", e);
                        "☢ I'm sorry, I lost the source! ☢".to_string()
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

fn interpret_channel_mention(mention: String) -> Result<u64, ErrorKind> {
    match mention.strip_prefix("<#") {
        Some(ention) => {                       // Get it? "mention" with the prefix removed
            match ention.strip_suffix(">") {
                Some(entio) => {                // And then with the suffix removed
                    if let Ok(id) = entio.parse::<u64>() {
                        Ok(id)
                    } else {
                        Err(ErrorKind::InvalidInput)
                    }
                },
                None => Err(ErrorKind::InvalidInput)
            }
        },
        None => Err(ErrorKind::InvalidInput)
    }
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
            "log <#654644643515>"
        ];
        
        assert_eq!(split_message(&input[0]), ["roll", "2d6"]);
        assert_eq!(split_message(&input[1]), ["ping"]);
        assert_eq!(split_message(&input[2]), ["roll", "apple"]);
        assert_eq!(split_message(&input[3]), ["roll", "2d6", "asdf"]);
        assert_eq!(split_message(&input[4]), ["roll!", "2d6", "asdfdsa!"]);
        assert_eq!(split_message(&input[5]), ["roll", "2d6"]);
        assert_eq!(split_message(&input[6]), [""]);
        assert_eq!(split_message(&input[7]), ["log", "<#654644643515>"]);
    }

    #[test]
    fn channel_id_test() {
        let id: u64 = 654644643515;
        let channel_mention = "<#654644643515>".to_string();
        let bad_mention = "654644643515".to_string();

        assert_eq!(interpret_channel_mention(channel_mention), Ok(id));
        assert_eq!(interpret_channel_mention(bad_mention), Err(ErrorKind::InvalidInput));
    }
}