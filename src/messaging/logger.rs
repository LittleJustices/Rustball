use std::fs;
use std::fs::{ File, OpenOptions };
use std::io;
use std::io::Write;
use std::path::Path;

use serenity::model::channel::Message;

#[derive(Debug)]
pub struct Logger {
    pub log_path: String,
    log_file: File,
}

impl Logger {
    pub fn new(folder: &String, filename: &String) -> io::Result<Logger> {
        let log_path = format!("{}/{}.txt", folder, filename);
        fs::create_dir_all(folder)?;

        let mut log_file = OpenOptions::new()
                        .create_new(true)
                        .append(true)
                        .open(Path::new(&log_path))?;

        writeln!(log_file, "---LOG START---")?;

        Ok(Logger{ log_path, log_file })
    }

    pub fn record(&self, msg: &Message) -> io::Result<()> {
        let mut log = &self.log_file;
        let log_entry = format!("{} {}: {}", msg.timestamp.format("%Y-%m-%d %H:%M:%S"), msg.author.name, msg.content);

        writeln!(log, "{}", log_entry)?;

        Ok(())
    }

    pub fn end_log(&self) -> io::Result<String> {
        let mut file = &self.log_file;
        writeln!(file, "---LOG END---")?;
        let path = format!("{}", self.log_path);
        Ok(path)
    }
}