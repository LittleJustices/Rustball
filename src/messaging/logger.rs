use std::fs;
use std::fs::{ File, OpenOptions };
use std::io;
use std::io::Write;
use std::path::PathBuf;

use chrono::{DateTime, Utc};

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
        log_path.set_extension("md");
        println!("{:?}", log_path);

        let mut log_file = OpenOptions::new()
                        .create_new(true)
                        .append(true)
                        .open(&log_path)?;

        writeln!(log_file, "# LOG START\n")?;

        Ok(Logger{ log_path, log_file })
    }

    pub fn record(&self, timestamp: DateTime<Utc>, sender_name: &str, content: &str) -> io::Result<()> {
        let mut log = &self.log_file;
        let log_entry = format!(
            "###### {timestamp} {name}\n> {content}\n",
            timestamp=timestamp.format("[%H:%M:%S]"),
            name=sender_name,
            content=content.replace("\n", "\n> ")
        );

        writeln!(log, "{}", log_entry)?;

        Ok(())
    }

    pub fn end_log(&self) -> io::Result<String> {
        let mut file = &self.log_file;
        writeln!(file, "# LOG END")?;
        let path = format!("{}", self.log_path.display());
        Ok(path)
    }
}