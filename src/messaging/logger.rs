use std::fs;
use std::fs::{ File, OpenOptions };
use std::io;
use std::io::Write;
use std::path::{PathBuf};

use serenity::model::channel::Message;

#[derive(Debug)]
pub struct Logger {
    pub log_path: PathBuf,
    log_file: File,
}

impl Logger {
    pub fn new(folder: &String, filename: &String) -> io::Result<Logger> {
        let mut log_path = PathBuf::from(folder);
        fs::create_dir_all(&log_path)?;

        log_path.push(filename);
        log_path.set_extension("txt");
        println!("{:?}", log_path);

        let mut log_file = OpenOptions::new()
                        .create_new(true)
                        .append(true)
                        .open(&log_path)?;

        writeln!(log_file, "---LOG START---")?;

        Ok(Logger{ log_path, log_file })
    }

    pub fn record(&self, msg: &Message) -> io::Result<()> {
        let mut log = &self.log_file;
        let sender_name = match &msg.member {
            Some(m) => {
                if let Some(name) = &m.nick {
                    name
                } else {
                    &msg.author.name
                }
            },
            None => &msg.author.name
        };
        let log_entry = format!("{} {}: {}", msg.timestamp.format("%Y-%m-%d %H:%M:%S"), sender_name, msg.content);

        writeln!(log, "{}", log_entry)?;

        Ok(())
    }

    pub fn end_log(&self) -> io::Result<String> {
        let mut file = &self.log_file;
        writeln!(file, "---LOG END---")?;
        let path = format!("{}", self.log_path.display());
        Ok(path)
    }
}