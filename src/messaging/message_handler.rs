use serenity::{
    async_trait,
    model::{
        channel::Message, 
        gateway::Ready,
    },
    prelude::*,
    // utils::MessageBuilder,
};

pub struct Handler;

const PREFIX: char = '!';   // Prefix for messages Sixball will parse

#[async_trait]
impl EventHandler for Handler {
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }

    async fn message(&self, ctx: Context, msg: Message) {
        match msg.content.trim().strip_prefix(PREFIX) {
            Some(_) => return,
            None => {
                let mut log_data = ctx.data.write().await;
                let log_map = log_data
                                .get_mut::<crate::LogsKey>()
                                .expect("Failed to retrieve logs map!")
                                .lock().await;

                if let Some(log) = log_map.get(&msg.channel_id) {
                    match log.record(msg) {
                        Ok(_) => return,
                        Err(why) => println!("{}", why)
                    }
                }
                return;
            }
        }
    }
}

impl Handler {
    pub fn new() -> Handler {
        Handler
    }
}

/*
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
}
*/