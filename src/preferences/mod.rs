use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use directories::ProjectDirs;
use std::fs;
use std::io::{self, Read, Write};
use std::time::{Duration, Instant};
use log::{error, warn, info, debug};

/// Configuration structure for user preferences
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreferencesConfig {
    /// Selected theme name
    pub theme: String,
    /// Volume level (0-100)
    pub volume: u8,
    /// Last accessed directory
    pub last_directory: PathBuf,
}

impl Default for PreferencesConfig {
    fn default() -> Self {
        Self {
            theme: "monokai".to_string(), // Default theme
            volume: 50,                    // Default volume
            last_directory: PathBuf::new(),
        }
    }
}

impl PreferencesConfig {
    /// Load preferences from the system configuration file
    pub fn load() -> io::Result<Self> {
        let path = ensure_preferences_dir()?;
        
        // If file doesn't exist, return default config
        if !path.exists() {
            debug!("Preferences file not found, using defaults");
            return Ok(Self::default());
        }
        
        // Read the file content
        let mut file = fs::File::open(&path).map_err(|e| {
            error!("Failed to open preferences file at {:?}: {}", path, e);
            e
        })?;
        let mut contents = String::new();
        file.read_to_string(&mut contents).map_err(|e| {
            error!("Failed to read preferences file: {}", e);
            e
        })?;
        
        // Parse JSON
        serde_json::from_str(&contents).map_err(|e| {
            error!("Failed to parse preferences JSON: {}", e);
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to parse preferences: {}", e)
            )
        })
    }
    
    /// Save preferences to the system configuration file
    pub fn save(&self) -> io::Result<()> {
        let path = ensure_preferences_dir()?;
        
        // Serialize to JSON
        let contents = serde_json::to_string_pretty(self).map_err(|e| {
            error!("Failed to serialize preferences: {}", e);
            io::Error::new(
                io::ErrorKind::InvalidData,
                format!("Failed to serialize preferences: {}", e)
            )
        })?;
        
        // Write to file
        let mut file = fs::File::create(&path).map_err(|e| {
            error!("Failed to create preferences file at {:?}: {}", path, e);
            e
        })?;
        file.write_all(contents.as_bytes()).map_err(|e| {
            error!("Failed to write preferences data: {}", e);
            e
        })?;
        file.flush().map_err(|e| {
            error!("Failed to flush preferences file: {}", e);
            e
        })?;
        
        debug!("Successfully saved preferences to {:?}", path);
        Ok(())
    }
}

/// Manager for handling preferences state and persistence
#[derive(Debug)]
pub struct PreferencesManager {
    /// Current preferences configuration
    config: PreferencesConfig,
    /// Path to preferences file
    file_path: PathBuf,
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
    const RETRY_INTERVAL: Duration = Duration::from_secs(300); // 5 minutes
    const MAX_SAVE_FAILURES: u32 = 3;

    /// Creates a new PreferencesManager instance
    pub fn new() -> io::Result<Self> {
        let file_path = ensure_preferences_dir()?;
        let config = match PreferencesConfig::load() {
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
            file_path,
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

        match self.config.save() {
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

/// Get the system-specific path for preferences file
pub fn get_preferences_path() -> Option<PathBuf> {
    ProjectDirs::from("com", "playtui", "playtui").map(|proj_dirs| {
        let config_dir = proj_dirs.config_dir();
        config_dir.join("preferences.json")
    })
}

/// Ensure the preferences directory exists, creating it if necessary
pub fn ensure_preferences_dir() -> io::Result<PathBuf> {
    let path = get_preferences_path()
        .ok_or_else(|| {
            error!("Could not determine preferences directory");
            io::Error::new(
                io::ErrorKind::NotFound,
                "Could not determine preferences directory"
            )
        })?;
        
    // Get the parent directory of the preferences file
    let parent = path.parent().ok_or_else(|| {
        error!("Could not determine parent directory for {:?}", path);
        io::Error::new(
            io::ErrorKind::NotFound,
            "Could not determine parent directory"
        )
    })?;

    // Create all parent directories if they don't exist
    fs::create_dir_all(parent).map_err(|e| {
        error!("Failed to create preferences directory {:?}: {}", parent, e);
        e
    })?;
    
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::thread;
    use std::os::unix::fs::PermissionsExt;

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
    fn test_preferences_serialization() {
        let config = PreferencesConfig::default();
        
        // Test serialization
        let serialized = serde_json::to_string(&config).expect("Failed to serialize config");
        assert!(!serialized.is_empty());
        
        // Test deserialization
        let deserialized: PreferencesConfig = serde_json::from_str(&serialized)
            .expect("Failed to deserialize config");
            
        assert_eq!(deserialized.theme, "monokai");
        assert_eq!(deserialized.volume, 50);
        assert_eq!(deserialized.last_directory, PathBuf::new());
    }

    #[test]
    fn test_volume_bounds() {
        let config = PreferencesConfig {
            volume: 100,
            ..Default::default()
        };
        assert_eq!(config.volume, 100);

        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: PreferencesConfig = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.volume, 100);
    }

    #[test]
    fn test_custom_theme() {
        let config = PreferencesConfig {
            theme: "custom".to_string(),
            ..Default::default()
        };
        assert_eq!(config.theme, "custom");

        let serialized = serde_json::to_string(&config).unwrap();
        let deserialized: PreferencesConfig = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized.theme, "custom");
    }

    #[test]
    fn test_preferences_path() {
        let path = get_preferences_path();
        assert!(path.is_some());
        
        let path = path.unwrap();
        assert!(path.ends_with("preferences.json"));
        assert!(path.to_string_lossy().contains("playtui"));
    }

    #[test]
    fn test_ensure_preferences_dir() {
        let result = ensure_preferences_dir();
        assert!(result.is_ok());
        
        let path = result.unwrap();
        let parent = path.parent().unwrap();
        assert!(parent.exists());
    }

    #[test]
    fn test_save_and_load() {
        // Start with a clean state
        cleanup_preferences().unwrap();

        // Create a custom config
        let config = PreferencesConfig {
            theme: "save_load_test_theme".to_string(),
            volume: 75,
            last_directory: PathBuf::from("/test/path"),
        };
        
        // Save it
        config.save().unwrap();
        
        // Load it back
        let loaded = PreferencesConfig::load().unwrap();
        
        // Verify contents
        assert_eq!(loaded.theme, "save_load_test_theme");
        assert_eq!(loaded.volume, 75);
        assert_eq!(loaded.last_directory, PathBuf::from("/test/path"));

        // Clean up after test
        cleanup_preferences().unwrap();
    }

    #[test]
    fn test_preferences_manager() {
        // Start with a clean state
        cleanup_preferences().unwrap();

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
    fn test_unwritable_location_handling() {
        // Start with a clean state
        cleanup_preferences().unwrap();

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
    fn test_save_retry_timing() {
        // Start with a clean state
        cleanup_preferences().unwrap();

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
