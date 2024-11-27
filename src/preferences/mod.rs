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
        // Create a custom config
        let config = PreferencesConfig {
            theme: "test_theme".to_string(),
            volume: 75,
            last_directory: PathBuf::from("/test/path"),
        };
        
        // Save it
        assert!(config.save().is_ok());
        
        // Load it back
        let loaded = PreferencesConfig::load().unwrap();
        
        // Verify contents
        assert_eq!(loaded.theme, "test_theme");
        assert_eq!(loaded.volume, 75);
        assert_eq!(loaded.last_directory, PathBuf::from("/test/path"));
    }
}
