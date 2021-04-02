use std::io::ErrorKind;

static mut LOGGED_CHANNELS: Vec<u64> = Vec::new();

pub fn log_channel(chan: u64) -> Result<u64, ErrorKind> {
    unsafe {
        if LOGGED_CHANNELS.contains(&chan) {
            return Err(ErrorKind::AlreadyExists)
        } else {
            LOGGED_CHANNELS.push(chan);
            return Ok(chan);
        }
    }
}

pub fn unlog_channel(chan: u64) -> Result<u64, ErrorKind> {
    unsafe {
        if LOGGED_CHANNELS.contains(&chan) {
            LOGGED_CHANNELS.retain(|x| x != &chan);
            return Ok(chan);
        } else {
            return Err(ErrorKind::NotFound);
        }
    }
}

pub fn logging() -> Vec<u64> {
    unsafe { return LOGGED_CHANNELS.clone() }
}