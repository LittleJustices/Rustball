use std::collections::HashMap;
use std::fs::{ File, OpenOptions };
use std::io::{ ErrorKind, Write };
use std::sync::{ Arc, Mutex };

use chrono::prelude::*;

use serenity::{
    model::{
        channel::{
            Message,
            Channel,
        },
        id::ChannelId,
    },
    prelude::Context,
};

#[derive(Debug)]
pub struct Logger {
    logged_channels: Arc<Mutex<HashMap<u64, File>>>
}

impl Logger {
    pub fn new() -> Logger {
        Logger { logged_channels: Arc::new(Mutex::new(HashMap::<u64, File>::new())) }
    }

    pub async fn check_logging_permission(target: u64, source: ChannelId, ctx: &Context) -> bool {
        // Block permission by default
        let mut allowed: bool = false;

        // If the channel to be logged is the same as the one the command was issued in, allow (this includes DMs)
        if source == ChannelId(target) { allowed = true; }

        // Otherwise, get the target channel and source channel's guilds/servers:
        // First, convert both from ChannelId to Channel
        let (target_res, source_res) = (ChannelId(target).to_channel(ctx).await, source.to_channel(ctx).await);
        if let (Ok(target_chan), Ok(source_chan)) = (target_res, source_res) {
            // Check if target and source are both guild channels (i.e. not DMs or anything other than a channel in a server)
            if let (Channel::Guild(target_guild_channel), Channel::Guild(source_guild_channel)) = (target_chan, source_chan) {
                // Then check if both channels belong to the same guild. If so, allow
                if target_guild_channel.guild_id == source_guild_channel.guild_id { allowed = true; }
            }
        }

        // Return permission
        allowed
    }

    pub fn log_channel(&self, chan: u64) -> Result<u64, ErrorKind> {
        let channels = Arc::clone(&self.logged_channels);
        let mut channel_list = channels.lock().unwrap();
        if channel_list.contains_key(&chan) {
            return Err(ErrorKind::AlreadyExists)
        } else {
            // Get current time and date
            let log_start_time = Utc::now().format("%Y-%m-%d-%a_%H:%M:%S");
            let log_file_path = format!("Sixball_Log_{}.txt", log_start_time);
            let log_file_result = OpenOptions::new()
                                    .create_new(true)
                                    .write(true)
                                    .open(log_file_path);

            match log_file_result {
                Ok(mut file) => {
                    match writeln!(file, "---LOG START---") {
                        Ok(_) => {
                            channel_list.insert(chan, file);
                            println!("Logging channel {:?}", channel_list.get(&chan));
                            return Ok(chan);
                        },
                        Err(_) => return Err(ErrorKind::Other)
                    }
                },
                Err(_) => return Err(ErrorKind::AlreadyExists)
            }
        }
    }

    pub fn unlog_channel(&self, chan: u64) -> Result<u64, ErrorKind> {
        let channels = Arc::clone(&self.logged_channels);
        let mut channel_list = channels.lock().unwrap();
        if channel_list.contains_key(&chan) {
            if let Some((_c, mut file)) = channel_list.remove_entry(&chan) {
                match writeln!(file, "---LOG END---") {
                    Ok(_) => {
                        return Ok(chan);
                    },
                    Err(_) => return Err(ErrorKind::Other)
                }
            } else {
                return Err(ErrorKind::Other);
            }
        } else {
            return Err(ErrorKind::NotFound);
        }
    }

    pub fn logging(&self, chan: u64) -> bool {
        let channels = Arc::clone(&self.logged_channels);
        let channel_list = channels.lock().unwrap();
        return channel_list.contains_key(&chan);
    }

    pub fn record(&self, msg: Message) -> Result<(), ErrorKind> {
        let output = format!("{} {}: {}", msg.timestamp, msg.author.name, msg.content);
        println!("{}", output);
        let channels = Arc::clone(&self.logged_channels);
        let channel_list = channels.lock().unwrap();
        match channel_list.get(&msg.channel_id.0) {
            Some(mut file) => {
                match writeln!(file, "{}", output) {
                    Ok(_) => return Ok(()),
                    Err(_) => return Err(ErrorKind::Other)
                };
            }
            None => return Err(ErrorKind::NotFound)
        }
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    // Tests currently broken due to permission checking which requires 
    // connecting to the discord API, so can only be tested live
    // #[test]
    // fn logging_test() {
    //     let logger = Logger::new();
    //     let chan_id: u64 = 1;
    //     let logged = logger.log_channel(chan_id);
    //     assert_eq!(logged, Ok(1));
    //     assert!(logger.logging(1));
    //     let _other_logged = logger.log_channel(2);
    //     assert!(logger.logging(1));
    //     assert!(logger.logging(2));
    // }

    // #[test]
    // fn unlogging_test() {
    //     let logger = Logger::new();
    //     let _logged = logger.log_channel(1);
    //     let _other_logged = logger.log_channel(2);
    //     assert!(logger.logging(2));
    //     assert!(logger.logging(1));
    //     let _unlogged = logger.unlog_channel(1);
    //     assert!(logger.logging(2));
    //     assert!(!logger.logging(1));
    // }
}