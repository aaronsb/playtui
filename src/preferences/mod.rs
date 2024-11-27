use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use directories::ProjectDirs;
use std::fs;
use std::io::{self, Read, Write};

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
            return Ok(Self::default());
        }
        
        // Read the file content
        let mut file = fs::File::open(path)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        
        // Parse JSON
        serde_json::from_str(&contents).map_err(|e| io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to parse preferences: {}", e)
        ))
    }
    
    /// Save preferences to the system configuration file
    pub fn save(&self) -> io::Result<()> {
        let path = ensure_preferences_dir()?;
        
        // Serialize to JSON
        let contents = serde_json::to_string_pretty(self).map_err(|e| io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to serialize preferences: {}", e)
        ))?;
        
        // Write to file
        let mut file = fs::File::create(path)?;
        file.write_all(contents.as_bytes())?;
        file.flush()?;
        
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
}

impl PreferencesManager {
    /// Creates a new PreferencesManager instance
    pub fn new() -> io::Result<Self> {
        let file_path = ensure_preferences_dir()?;
        let config = PreferencesConfig::load()?;
        
        Ok(Self {
            config,
            file_path,
            dirty: false,
        })
    }
    
    /// Gets a reference to the current preferences configuration
    pub fn config(&self) -> &PreferencesConfig {
        &self.config
    }
    
    /// Updates the theme and marks preferences as dirty
    pub fn update_theme(&mut self, theme: String) {
        self.config.theme = theme;
        self.dirty = true;
    }
    
    /// Updates the volume level and marks preferences as dirty
    pub fn update_volume(&mut self, volume: u8) {
        self.config.volume = volume;
        self.dirty = true;
    }
    
    /// Updates the last accessed directory and marks preferences as dirty
    pub fn update_last_directory(&mut self, path: PathBuf) {
        self.config.last_directory = path;
        self.dirty = true;
    }
    
    /// Saves preferences if they have been modified since last save
    pub fn save_if_dirty(&mut self) -> io::Result<()> {
        if self.dirty {
            self.config.save()?;
            self.dirty = false;
        }
        Ok(())
    }
    
    /// Forces a save regardless of dirty state
    pub fn save(&mut self) -> io::Result<()> {
        self.config.save()?;
        self.dirty = false;
        Ok(())
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
        .ok_or_else(|| io::Error::new(
            io::ErrorKind::NotFound,
            "Could not determine preferences directory"
        ))?;
        
    // Get the parent directory of the preferences file
    let parent = path.parent().ok_or_else(|| io::Error::new(
        io::ErrorKind::NotFound,
        "Could not determine parent directory"
    ))?;

    // Create all parent directories if they don't exist
    fs::create_dir_all(parent)?;
    
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    fn cleanup_preferences() -> io::Result<()> {
        if let Some(path) = get_preferences_path() {
            if path.exists() {
                fs::remove_file(path)?;
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
        let new_manager = PreferencesManager::new().unwrap();
        assert_eq!(new_manager.config().theme, "manager_test_theme");
        assert_eq!(new_manager.config().volume, 75);
        assert_eq!(new_manager.config().last_directory, test_path);

        // Clean up after test
        cleanup_preferences().unwrap();
    }
}
