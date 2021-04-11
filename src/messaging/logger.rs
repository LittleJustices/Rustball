use std::collections::HashMap;
use std::fs::{ File, OpenOptions };
use std::io::{ ErrorKind, Write };
use std::path::Path;
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
    logged_channels: Arc<Mutex<HashMap<u64, (File, String)>>>
}

impl Logger {
    pub fn new() -> Logger {
        Logger { logged_channels: Arc::new(Mutex::new(HashMap::<u64, (File, String)>::new())) }
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

    pub fn log_channel(&self, chan: u64, filename: String) -> Result<u64, ErrorKind> {
        let channels = Arc::clone(&self.logged_channels);
        let mut channel_list = channels.lock().unwrap();
        if channel_list.contains_key(&chan) {
            return Err(ErrorKind::AlreadyExists)
        } else {
            let log_file_path = format!("./Logs/{}.txt", filename);
            let log_file_result = OpenOptions::new()
                                    .create_new(true)
                                    .write(true)
                                    .open(Path::new(&log_file_path));

            match log_file_result {
                Ok(mut file) => {
                    match writeln!(file, "---LOG START---") {
                        Ok(_) => {
                            channel_list.insert(chan, (file, log_file_path));
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

    pub fn unlog_channel(&self, chan: u64) -> Result<String, ErrorKind> {
        let channels = Arc::clone(&self.logged_channels);
        let mut channel_list = channels.lock().unwrap();
        if channel_list.contains_key(&chan) {
            if let Some((_c, (mut file, path))) = channel_list.remove_entry(&chan) {
                match writeln!(file, "---LOG END---") {
                    Ok(_) => {
                        return Ok(path);
                    },
                    Err(_) => return Err(ErrorKind::PermissionDenied)
                }
            } else {
                return Err(ErrorKind::TimedOut);
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
        let output = format!("{} {}: {}", msg.timestamp.format("%Y-%m-%d %H:%M:%S"), msg.author.name, msg.content);
        let channels = Arc::clone(&self.logged_channels);
        let channel_list = channels.lock().unwrap();
        let mut record;
        match channel_list.get(&msg.channel_id.0) {
            Some((file, _path)) => record = file,
            None => return Err(ErrorKind::NotFound)
        };
        match writeln!(record, "{}", output) {
            Ok(_) => return Ok(()),
            Err(_) => return Err(ErrorKind::Other)
        };
    }

    pub async fn construct_log_filename(id: u64, ctx: &Context) -> String {
        let mut chan_name = "no_chan_name".to_string();
        let mut guild_name = "".to_string();
        
        if let Ok(chan) = ChannelId(id).to_channel(ctx).await {
            if let Channel::Guild(guild_chan) = chan {
                chan_name = guild_chan.name().to_string();
                let guild = guild_chan.guild_id;
                guild_name = match guild.name(ctx).await {
                    Some(name) => format!("_{}", name),
                    None => "".to_string()
                }
            }
        }
        
        let log_start_time = Utc::now().format("%Y-%m-%d-%a_%H:%M:%S");
        let log_file_name = format!("Sixball_Log{}_{}_{}", guild_name, chan_name, log_start_time);
        return log_file_name;
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