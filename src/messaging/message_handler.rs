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
    logger::Logger,
};

pub struct Handler {
    pub responses: Can,
    pub log: Logger,
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
                if self.log.logging(msg.channel_id.0) {
                    println!("{} {}: {}", msg.timestamp, msg.author.name, msg.content);
                }
                return;
            }
        }
        let (command, argument, _comment) = split_message(content);
        match &command[..] {
            // Shutdown order
            "bye" => {
                let bye = "Bye~! ❤".to_string();

                self.send_msg(&ctx, msg.channel_id, bye).await;
                std::process::exit(0);
            },
            // Start logging a channel
            "log" => {
                let chan = match interpret_channel_mention(argument) {
                    Ok(c) => c,
                    Err(ErrorKind::Other) => msg.channel_id.0,
                    Err(_) => 0,
                };
                match self.log.log_channel(chan) {
                    Ok(c) => {
                        let log_confirm = format!("Logging <#{}> now! ❤", c);
                        self.send_msg(&ctx, ChannelId(chan), log_confirm).await;
                    },
                    Err(ErrorKind::AlreadyExists) => {
                        let log_error = "☢ I'm already logging that channel! ☢".to_string();
                        self.call_and_response(&ctx, msg, log_error).await;
                    },
                    Err(_) => {
                        let chan_error = "☢ That's not a channel I recognize! ☢".to_string();
                        self.call_and_response(&ctx, msg, chan_error).await;
                        return;
                    },
                }
            },
            // Stop logging a channel
            "unlog" => {
                let chan = match interpret_channel_mention(argument) {
                    Ok(c) => c,
                    Err(ErrorKind::Other) => msg.channel_id.0,
                    Err(_) => 0,
                };
                match self.log.unlog_channel(chan) {
                    Ok(c) => {
                        let log_confirm = format!("Okay, I'll stop logging <#{}>! ❤", c);
                        self.send_msg(&ctx, ChannelId(chan), log_confirm).await;
                    },
                    Err(ErrorKind::NotFound) => {
                        let log_error = "☢ I'm not logging that channel yet! ☢".to_string();
                        self.call_and_response(&ctx, msg, log_error).await;
                    },
                    Err(_) => {
                        let chan_error = "☢ That's not a channel I recognize! ☢".to_string();
                        self.call_and_response(&ctx, msg, chan_error).await;
                        return;
                    },
                }
            },
            // Check if a channel is currently being logged
            "logging" => {
                let chan = match interpret_channel_mention(argument) {
                    Ok(c) => c,
                    Err(ErrorKind::Other) => msg.channel_id.0,
                    Err(_) => 0,
                };
                if self.log.logging(chan) {
                    let yes = format!("I'm logging <#{}> right now!", chan);
                    self.call_and_response(&ctx, msg, yes).await;
                } else {
                    let no = format!("I'm not logging <#{}> right now!", chan);
                    self.call_and_response(&ctx, msg, no).await;
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
                if let Ok(ans) = self.responses.find_in_can(&command) {
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

fn split_message<'a>(message: &'a str) -> (String, String, String) {
    // Create string vector to hold the content
    let (mut command, mut argument, mut comment) = ("".to_string(), "".to_string(), "".to_string());

    // If the input is empty, just push and return the empty string
    if message == "" {
        return (command, argument, comment);
    }

    // Then, split along spaces
    let mut chunks = message.split_whitespace();

    // First element, if present, is the command. This can never be None, but if it is, that's an empty string
    command = match chunks.next() {
        Some(c) => c.to_lowercase(),
        None => "".to_string()
    };

    // Second element, if present, is the argument
    argument = match chunks.next() {
        Some(c) => c.to_lowercase(),
        None => "".to_string()
    };

    // Anything that's left should be collected into a single string and become the comment
    let mut leftover_text = "".to_string();

    // Loop over the first two chunks and push to content
    for chunk in chunks {
        leftover_text = format!("{} {}", leftover_text, chunk);
    }

    // Push leftovers to content as a single string, if not empty
    comment = leftover_text.trim().to_string();
    
    // return content
    (command, argument, comment)
}

fn interpret_channel_mention(mention: String) -> Result<u64, ErrorKind> {
    if mention == "".to_string() { return Err(ErrorKind::Other) }
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
            "roll! 2d6 Never Gonna Give You Up",
            "RoLl 2D6",
            "",
            "log <#654644643515>"
        ];
        
        assert_eq!(split_message(&input[0]), ("roll".to_string(), "2d6".to_string(), "".to_string()));
        assert_eq!(split_message(&input[1]), ("ping".to_string(), "".to_string(), "".to_string()));
        assert_eq!(split_message(&input[2]), ("roll".to_string(), "apple".to_string(), "".to_string()));
        assert_eq!(split_message(&input[3]), ("roll".to_string(), "2d6".to_string(), "asdf".to_string()));
        assert_eq!(split_message(&input[4]), ("roll!".to_string(), "2d6".to_string(), "Never Gonna Give You Up".to_string()));
        assert_eq!(split_message(&input[5]), ("roll".to_string(), "2d6".to_string(), "".to_string()));
        assert_eq!(split_message(&input[6]), ("".to_string(), "".to_string(), "".to_string()));
        assert_eq!(split_message(&input[7]), ("log".to_string(), "<#654644643515>".to_string(), "".to_string()));
    }

    #[test]
    fn channel_id_test() {
        let id: u64 = 4646516846168;
        let channel_mention = "<#4646516846168>".to_string();
        let bad_mention = "apple".to_string();
        let no_mention = "".to_string();

        assert_eq!(interpret_channel_mention(channel_mention), Ok(id));
        assert_eq!(interpret_channel_mention(bad_mention), Err(ErrorKind::InvalidInput));
        assert_eq!(interpret_channel_mention(no_mention), Err(ErrorKind::Other));
    }
}