use std::path::PathBuf;
use std::io;
use log::{warn, info, debug};

use crate::preferences::config::PreferencesConfig;
use crate::preferences::persistence;

mod save_handler;
#[cfg(test)]
mod tests;

use save_handler::SaveHandler;

/// Manager for handling preferences state and persistence
#[derive(Debug)]
pub struct PreferencesManager {
    /// Current preferences configuration
    config: PreferencesConfig,
    /// Whether preferences have been modified since last save
    dirty: bool,
    /// Handler for save operations and filesystem state
    save_handler: SaveHandler,
}

impl PreferencesManager {
    /// Creates a new PreferencesManager instance
    pub fn new() -> io::Result<Self> {
        let config = match persistence::load_preferences() {
            Ok(config) => {
                info!("Successfully loaded preferences");
                config
            },
            Err(e) => {
                warn!("Failed to load preferences, using defaults: {}", e);
                PreferencesConfig::default()
            }
        };
        
        Ok(Self {
            config,
            dirty: false,
            save_handler: SaveHandler::new(),
        })
    }
    
    /// Gets a reference to the current preferences configuration
    pub fn config(&self) -> &PreferencesConfig {
        &self.config
    }
    
    /// Updates the theme and marks preferences as dirty
    pub fn update_theme(&mut self, theme: String) {
        debug!("Updating theme to: {}", theme);
        self.config.theme = theme;
        self.dirty = true;
    }
    
    /// Updates the volume level and marks preferences as dirty
    pub fn update_volume(&mut self, volume: u8) {
        debug!("Updating volume to: {}", volume);
        self.config.volume = volume;
        self.dirty = true;
    }
    
    /// Updates the last accessed directory and marks preferences as dirty
    pub fn update_last_directory(&mut self, path: PathBuf) {
        debug!("Updating last directory to: {:?}", path);
        self.config.last_directory = path;
        self.dirty = true;
    }
    
    /// Saves preferences if they have been modified since last save
    pub fn save_if_dirty(&mut self) -> io::Result<()> {
        if !self.dirty {
            return Ok(());
        }

        if self.save_handler.should_skip_save() {
            debug!("Filesystem marked unwritable and retry interval not reached, skipping save");
            return Ok(());
        }

        match self.save_handler.attempt_save(&self.config) {
            Ok(()) => {
                self.dirty = false;
                Ok(())
            }
            Err(e) => Err(e)
        }
    }
    
    /// Forces a save regardless of dirty state
    pub fn save(&mut self) -> io::Result<()> {
        if self.save_handler.should_skip_save() {
            debug!("Filesystem marked unwritable and retry interval not reached, skipping forced save");
            return Ok(());
        }

        match self.save_handler.attempt_save(&self.config) {
            Ok(()) => {
                self.dirty = false;
                Ok(())
            }
            Err(e) => Err(e)
        }
    }
}
