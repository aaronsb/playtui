use serde::{Deserialize, Serialize};
use std::path::PathBuf;

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
}
