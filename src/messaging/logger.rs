use std::io::ErrorKind;
use std::sync::{Arc, Mutex};

use serenity::model::channel::Message;

#[derive(Debug)]
pub struct Logger {
    logged_channels: Arc<Mutex<Vec<u64>>>
}

impl Logger {
    pub fn new() -> Logger {
        Logger { logged_channels: Arc::new(Mutex::new(Vec::<u64>::new())) }
    }

    pub fn log_channel(&self, chan: u64) -> Result<u64, ErrorKind> {
        if chan == 0 { return Err(ErrorKind::InvalidInput) }
        let channels = Arc::clone(&self.logged_channels);
        let mut channel_list = channels.lock().unwrap();
        if channel_list.contains(&chan) {
            return Err(ErrorKind::AlreadyExists)
        } else {
            channel_list.push(chan);
            return Ok(chan);
        }
    }

    pub fn unlog_channel(&self, chan: u64) -> Result<u64, ErrorKind> {
        if chan == 0 { return Err(ErrorKind::InvalidInput) }
        let channels = Arc::clone(&self.logged_channels);
        let mut channel_list = channels.lock().unwrap();
        if channel_list.contains(&chan) {
            channel_list.retain(|x| x != &chan);
            return Ok(chan);
        } else {
            return Err(ErrorKind::NotFound);
        }
    }

    pub fn logging(&self, chan: u64) -> bool {
        let channels = Arc::clone(&self.logged_channels);
        let channel_list = channels.lock().unwrap();
        return channel_list.contains(&chan);
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