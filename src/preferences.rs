use std::path::PathBuf;
use std::fs;
use serde::{Deserialize, Serialize};
use anyhow::{Context, Result};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub struct Preferences {
    pub theme: String,
    pub volume: f32,
    pub repeat_mode: bool,
    pub last_directory: Option<String>,
}

impl Default for Preferences {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
            volume: 1.0,
            repeat_mode: false,
            last_directory: None,
        }
    }
}

#[derive(Debug)]
pub enum PreferencesError {
    FileCorrupted(String),
    IoError(std::io::Error),
}

impl std::fmt::Display for PreferencesError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PreferencesError::FileCorrupted(msg) => write!(f, "Preferences file is corrupted: {}", msg),
            PreferencesError::IoError(err) => write!(f, "IO error: {}", err),
        }
    }
}

impl std::error::Error for PreferencesError {}

// Manual implementation of PartialEq that compares error kinds for IoError
impl PartialEq for PreferencesError {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (PreferencesError::FileCorrupted(a), PreferencesError::FileCorrupted(b)) => a == b,
            (PreferencesError::IoError(a), PreferencesError::IoError(b)) => a.kind() == b.kind(),
            _ => false,
        }
    }
}

impl Preferences {
    pub fn load() -> Result<(Self, Option<PreferencesError>)> {
        let config_dir = dirs::home_dir()
            .map(|h| h.join(".config").join("playtui"))
            .context("Could not determine home directory")?;

        fs::create_dir_all(&config_dir)
            .context("Failed to create config directory")?;

        let config_path = config_dir.join("preferences.json");

        if !config_path.exists() {
            let prefs = Self::default();
            prefs.save()?;
            return Ok((prefs, None));
        }

        match fs::read_to_string(&config_path) {
            Ok(contents) => match serde_json::from_str(&contents) {
                Ok(prefs) => Ok((prefs, None)),
                Err(e) => Ok((
                    Self::default(),
                    Some(PreferencesError::FileCorrupted(e.to_string())),
                )),
            },
            Err(e) => Ok((
                Self::default(),
                Some(PreferencesError::IoError(e)),
            )),
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_dir = dirs::home_dir()
            .map(|h| h.join(".config").join("playtui"))
            .context("Could not determine home directory")?;

        fs::create_dir_all(&config_dir)
            .context("Failed to create config directory")?;

        let config_path = config_dir.join("preferences.json");
        let contents = serde_json::to_string_pretty(self)
            .context("Failed to serialize preferences")?;

        fs::write(&config_path, contents)
            .context("Failed to write preferences file")?;

        Ok(())
    }

    pub fn config_path() -> Option<PathBuf> {
        dirs::home_dir().map(|h| h.join(".config").join("playtui").join("preferences.json"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;
    use std::env;

    #[test]
    fn test_preferences_default() {
        let prefs = Preferences::default();
        assert_eq!(prefs.theme, "default");
        assert_eq!(prefs.volume, 1.0);
        assert_eq!(prefs.repeat_mode, false);
        assert_eq!(prefs.last_directory, None);
    }

    #[test]
    fn test_preferences_serialization() {
        let prefs = Preferences {
            theme: "dark".to_string(),
            volume: 0.8,
            repeat_mode: true,
            last_directory: Some("/music".to_string()),
        };

        let serialized = serde_json::to_string_pretty(&prefs).unwrap();
        let deserialized: Preferences = serde_json::from_str(&serialized).unwrap();
        assert_eq!(prefs, deserialized);
    }

    #[test]
    fn test_preferences_load_corrupted() {
        // Create a temporary directory
        let temp_dir = tempdir().unwrap();
        let config_dir = temp_dir.path().join(".config").join("playtui");
        fs::create_dir_all(&config_dir).unwrap();
        
        // Create a corrupted preferences file
        let config_path = config_dir.join("preferences.json");
        fs::write(&config_path, "{corrupted json}").unwrap();

        // Temporarily override HOME directory
        env::set_var("HOME", temp_dir.path());

        // Load preferences
        let (prefs, error) = Preferences::load().unwrap();
        
        // Should return default preferences and a corruption error
        assert_eq!(prefs, Preferences::default());
        assert!(matches!(error, Some(PreferencesError::FileCorrupted(_))));
    }

    #[test]
    fn test_preferences_save_and_load() {
        // Create a temporary directory
        let temp_dir = tempdir().unwrap();
        
        // Temporarily override HOME directory
        env::set_var("HOME", temp_dir.path());

        // Create and save preferences
        let prefs = Preferences {
            theme: "monokai".to_string(),
            volume: 0.5,
            repeat_mode: true,
            last_directory: Some("/test".to_string()),
        };
        prefs.save().unwrap();

        // Load preferences
        let (loaded_prefs, error) = Preferences::load().unwrap();
        
        // Verify loaded preferences match saved preferences
        assert_eq!(prefs, loaded_prefs);
        assert!(error.is_none());
    }

    #[test]
    fn test_preferences_error_equality() {
        // Test FileCorrupted equality
        let err1 = PreferencesError::FileCorrupted("test error".to_string());
        let err2 = PreferencesError::FileCorrupted("test error".to_string());
        let err3 = PreferencesError::FileCorrupted("different error".to_string());
        assert_eq!(err1, err2);
        assert_ne!(err1, err3);

        // Test IoError equality
        use std::io::{Error, ErrorKind};
        let io_err1 = PreferencesError::IoError(Error::new(ErrorKind::NotFound, "error"));
        let io_err2 = PreferencesError::IoError(Error::new(ErrorKind::NotFound, "different message"));
        let io_err3 = PreferencesError::IoError(Error::new(ErrorKind::PermissionDenied, "error"));
        assert_eq!(io_err1, io_err2); // Same kind should be equal
        assert_ne!(io_err1, io_err3); // Different kinds should not be equal

        // Test different variants are not equal
        assert_ne!(err1, io_err1);
    }
}
