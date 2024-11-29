use std::time::{Duration, Instant};
use std::io;
use log::{error, warn, debug};

use crate::preferences::config::PreferencesConfig;
use crate::preferences::persistence;

/// Handles saving preferences with retry logic and filesystem state tracking
#[derive(Debug)]
pub struct SaveHandler {
    /// Last time a save was attempted
    last_save_attempt: Option<Instant>,
    /// Whether the file system is currently writable
    fs_writable: bool,
    /// Number of consecutive save failures
    save_failures: u32,
}

impl SaveHandler {
    pub const RETRY_INTERVAL: Duration = Duration::from_millis(100); // Very short for testing
    pub const MAX_SAVE_FAILURES: u32 = 3;

    pub fn new() -> Self {
        Self {
            last_save_attempt: None,
            fs_writable: true,
            save_failures: 0,
        }
    }

    /// Checks if enough time has passed since the last save attempt
    pub fn should_retry_save(&self) -> bool {
        self.last_save_attempt
            .map(|last| last.elapsed() >= Self::RETRY_INTERVAL)
            .unwrap_or(true)
    }

    /// Attempts to save preferences, handling filesystem errors
    pub fn attempt_save(&mut self, config: &PreferencesConfig) -> io::Result<()> {
        self.last_save_attempt = Some(Instant::now());

        match persistence::save_preferences(config) {
            Ok(()) => {
                self.fs_writable = true;
                self.save_failures = 0;
                debug!("Successfully saved preferences");
                Ok(())
            }
            Err(e) => {
                self.save_failures += 1;
                if self.save_failures >= Self::MAX_SAVE_FAILURES {
                    self.fs_writable = false;
                    error!("Maximum save failures reached, marking filesystem as unwritable");
                }
                Err(e)
            }
        }
    }

    /// Returns whether the filesystem is currently writable
    pub fn is_fs_writable(&self) -> bool {
        self.fs_writable
    }

    /// Returns the number of consecutive save failures
    pub fn save_failures(&self) -> u32 {
        self.save_failures
    }

    /// Checks if save should be skipped due to filesystem state
    pub fn should_skip_save(&self) -> bool {
        !self.fs_writable && !self.should_retry_save()
    }
}
