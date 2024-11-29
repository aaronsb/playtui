use std::path::PathBuf;
use std::time::{Duration, Instant};
use std::io;
use log::{error, warn, info, debug};

use super::config::PreferencesConfig;
use super::persistence;

/// Manager for handling preferences state and persistence
#[derive(Debug)]
pub struct PreferencesManager {
    /// Current preferences configuration
    config: PreferencesConfig,
    /// Whether preferences have been modified since last save
    dirty: bool,
    /// Last time a save was attempted
    last_save_attempt: Option<Instant>,
    /// Whether the file system is currently writable
    fs_writable: bool,
    /// Number of consecutive save failures
    save_failures: u32,
}

impl PreferencesManager {
    const RETRY_INTERVAL: Duration = Duration::from_millis(100); // Very short for testing
    const MAX_SAVE_FAILURES: u32 = 3;

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
            last_save_attempt: None,
            fs_writable: true,
            save_failures: 0,
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

    /// Checks if enough time has passed since the last save attempt
    fn should_retry_save(&self) -> bool {
        self.last_save_attempt
            .map(|last| last.elapsed() >= Self::RETRY_INTERVAL)
            .unwrap_or(true)
    }

    /// Attempts to save preferences, handling filesystem errors
    fn attempt_save(&mut self) -> io::Result<()> {
        self.last_save_attempt = Some(Instant::now());

        match persistence::save_preferences(&self.config) {
            Ok(()) => {
                self.fs_writable = true;
                self.save_failures = 0;
                self.dirty = false;
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
    
    /// Saves preferences if they have been modified since last save
    pub fn save_if_dirty(&mut self) -> io::Result<()> {
        if !self.dirty {
            return Ok(());
        }

        if !self.fs_writable && !self.should_retry_save() {
            debug!("Filesystem marked unwritable and retry interval not reached, skipping save");
            return Ok(());
        }

        self.attempt_save()
    }
    
    /// Forces a save regardless of dirty state
    pub fn save(&mut self) -> io::Result<()> {
        if !self.fs_writable && !self.should_retry_save() {
            debug!("Filesystem marked unwritable and retry interval not reached, skipping forced save");
            return Ok(());
        }

        self.attempt_save()
    }

    /// Returns whether the filesystem is currently writable
    pub fn is_fs_writable(&self) -> bool {
        self.fs_writable
    }

    /// Returns the number of consecutive save failures
    pub fn save_failures(&self) -> u32 {
        self.save_failures
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::thread;
    use std::os::unix::fs::PermissionsExt;
    use super::super::persistence::get_preferences_path;
    use serial_test::serial;

    fn setup_test_env() -> io::Result<()> {
        cleanup_preferences()?;
        if let Some(path) = get_preferences_path() {
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
                fs::set_permissions(parent, fs::Permissions::from_mode(0o755))?;
            }
        }
        Ok(())
    }

    fn cleanup_preferences() -> io::Result<()> {
        if let Some(preferences_path) = get_preferences_path() {
            if preferences_path.exists() {
                fs::remove_file(&preferences_path)?;
            }
            if let Some(parent) = preferences_path.parent() {
                if parent.exists() {
                    let _ = fs::set_permissions(parent, fs::Permissions::from_mode(0o755));
                }
            }
        }
        Ok(())
    }

    #[test]
    #[serial]
    fn test_preferences_manager() {
        // Setup test environment
        setup_test_env().unwrap();

        // Create new manager
        let mut manager = PreferencesManager::new().unwrap();
        
        // Test initial state
        assert!(!manager.dirty);
        assert_eq!(manager.config().theme, "monokai");
        
        // Test update operations
        manager.update_theme("manager_test_theme".to_string());
        assert!(manager.dirty);
        assert_eq!(manager.config().theme, "manager_test_theme");
        
        manager.update_volume(75);
        assert!(manager.dirty);
        assert_eq!(manager.config().volume, 75);
        
        let test_path = PathBuf::from("/test/path");
        manager.update_last_directory(test_path.clone());
        assert!(manager.dirty);
        assert_eq!(manager.config().last_directory, test_path);
        
        // Test save operations
        assert!(manager.save().is_ok());
        assert!(!manager.dirty);
        
        // Verify saved state persists
        drop(manager); // Ensure manager is dropped before creating new one
        let new_manager = PreferencesManager::new().unwrap();
        assert_eq!(new_manager.config().theme, "manager_test_theme");
        assert_eq!(new_manager.config().volume, 75);
        assert_eq!(new_manager.config().last_directory, test_path);

        // Clean up after test
        cleanup_preferences().unwrap();
    }

    #[test]
    #[serial]
    fn test_unwritable_location_handling() {
        // Setup test environment
        setup_test_env().unwrap();

        let mut manager = PreferencesManager::new().unwrap();
        
        // Simulate filesystem becoming unwritable by maxing out save failures
        for _ in 0..PreferencesManager::MAX_SAVE_FAILURES {
            // Make the directory read-only to force save failures
            if let Some(path) = get_preferences_path() {
                if let Some(parent) = path.parent() {
                    let _ = fs::set_permissions(parent, fs::Permissions::from_mode(0o444));
                }
            }
            
            manager.update_theme("test".to_string());
            let _ = manager.save();
        }

        // Verify filesystem is marked as unwritable
        assert!(!manager.is_fs_writable());
        assert_eq!(manager.save_failures(), PreferencesManager::MAX_SAVE_FAILURES);

        // Verify in-memory operations still work
        manager.update_volume(80);
        assert_eq!(manager.config().volume, 80);

        // Restore write permissions for cleanup
        if let Some(path) = get_preferences_path() {
            if let Some(parent) = path.parent() {
                let _ = fs::set_permissions(parent, fs::Permissions::from_mode(0o755));
            }
        }

        // Clean up after test
        cleanup_preferences().unwrap();
    }

    #[test]
    #[serial]
    fn test_save_retry_timing() {
        // Setup test environment
        setup_test_env().unwrap();

        let mut manager = PreferencesManager::new().unwrap();
        
        // Make the directory read-only to force save failures
        if let Some(path) = get_preferences_path() {
            if let Some(parent) = path.parent() {
                let _ = fs::set_permissions(parent, fs::Permissions::from_mode(0o444));
            }
        }

        // Force filesystem to be marked unwritable
        for _ in 0..PreferencesManager::MAX_SAVE_FAILURES {
            manager.update_theme("test".to_string());
            let _ = manager.save();
        }

        // Verify immediate retry is skipped
        assert!(!manager.is_fs_writable());
        let save_result = manager.save();
        assert!(save_result.is_ok()); // Save should "succeed" but actually be skipped

        // Wait for retry interval
        thread::sleep(PreferencesManager::RETRY_INTERVAL);

        // Verify save is attempted after interval
        manager.update_theme("retry_test".to_string());
        let save_result = manager.save();
        assert!(save_result.is_err()); // Should actually try to save and fail

        // Restore permissions and clean up
        if let Some(path) = get_preferences_path() {
            if let Some(parent) = path.parent() {
                let _ = fs::set_permissions(parent, fs::Permissions::from_mode(0o755));
            }
        }
        cleanup_preferences().unwrap();
    }
}
