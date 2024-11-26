use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::Path;
use std::time::{SystemTime, UNIX_EPOCH};
use crate::events::Event;

pub struct Logger {
    file: File,
}

impl Logger {
    pub fn new() -> std::io::Result<Self> {
        // Create logs directory if it doesn't exist
        let logs_dir = Path::new("logs");
        if !logs_dir.exists() {
            std::fs::create_dir(logs_dir)?;
        }

        // Create or open log file with timestamp in name
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        let log_file = format!("logs/events_{}.log", timestamp);
        
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(log_file)?;

        Ok(Logger { file })
    }

    pub fn log_event(&mut self, event: &Event) -> std::io::Result<()> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let log_entry = format!("[{}] Event: {:?}\n", timestamp, event);
        self.file.write_all(log_entry.as_bytes())?;
        self.file.flush()?;
        
        Ok(())
    }
}
