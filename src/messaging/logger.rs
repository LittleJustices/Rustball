use std::collections::HashMap;
use std::io::ErrorKind;
use std::sync::{Arc, Mutex};

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
    logged_channels: Arc<Mutex<HashMap<u64, String>>>
}

impl Logger {
    pub fn new() -> Logger {
        Logger { logged_channels: Arc::new(Mutex::new(HashMap::<u64, String>::new())) }
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
            channel_list.insert(chan, "log file name".to_string());
            println!("Logging channel {:?}", channel_list.get(&chan));
            return Ok(chan);
        }
    }

    pub fn unlog_channel(&self, chan: u64) -> Result<u64, ErrorKind> {
        let channels = Arc::clone(&self.logged_channels);
        let mut channel_list = channels.lock().unwrap();
        if channel_list.contains_key(&chan) {
            channel_list.remove_entry(&chan);
            return Ok(chan);
        } else {
            return Err(ErrorKind::NotFound);
        }
    }

    pub fn logging(&self, chan: u64) -> bool {
        let channels = Arc::clone(&self.logged_channels);
        let channel_list = channels.lock().unwrap();
        return channel_list.contains_key(&chan);
    }

    pub fn record(&self, msg: Message) {
        println!("{} {}: {}", msg.timestamp, msg.author.name, msg.content);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn logging_test() {
        let logger = Logger::new();
        let chan_id: u64 = 1;
        let logged = logger.log_channel(chan_id);
        assert_eq!(logged, Ok(1));
        assert!(logger.logging(1));
        let _other_logged = logger.log_channel(2);
        assert!(logger.logging(1));
        assert!(logger.logging(2));
    }

    #[test]
    fn unlogging_test() {
        let logger = Logger::new();
        let _logged = logger.log_channel(1);
        let _other_logged = logger.log_channel(2);
        assert!(logger.logging(2));
        assert!(logger.logging(1));
        let _unlogged = logger.unlog_channel(1);
        assert!(logger.logging(2));
        assert!(!logger.logging(1));
    }
}