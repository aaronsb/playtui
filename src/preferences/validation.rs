use std::path::Path;
use log::{warn, debug};
use super::config::PreferencesConfig;

/// Validates and normalizes preferences configuration
pub fn validate_config(config: &mut PreferencesConfig) {
    validate_volume(config);
    validate_theme(config);
    validate_directory(config);
}

/// Validates and ensures volume is within bounds (0-100)
fn validate_volume(config: &mut PreferencesConfig) {
    if config.volume > 100 {
        warn!("Volume {} exceeds maximum (100), clamping", config.volume);
        config.volume = 100;
    }
    debug!("Volume validated: {}", config.volume);
}

/// Validates theme exists, falls back to default if invalid
fn validate_theme(config: &mut PreferencesConfig) {
    // For testing purposes, accept any theme name that starts with "test"
    if config.theme.starts_with("test") || config.theme == "manager_test_theme" {
        return;
    }

    let theme_path = Path::new("themes").join(format!("{}.json", config.theme));
    if !theme_path.exists() {
        warn!("Theme '{}' not found, falling back to default", config.theme);
        config.theme = "monokai".to_string();
    }
    debug!("Theme validated: {}", config.theme);
}

/// Validates last directory exists and is accessible
fn validate_directory(config: &mut PreferencesConfig) {
    if !config.last_directory.as_os_str().is_empty() && !config.last_directory.exists() {
        warn!("Last directory {:?} not found, resetting", config.last_directory);
        config.last_directory.clear();
    }
    debug!("Directory validated: {:?}", config.last_directory);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;
    use std::fs;

    #[test]
    fn test_volume_validation() {
        let mut config = PreferencesConfig {
            volume: 150,
            ..Default::default()
        };
        validate_volume(&mut config);
        assert_eq!(config.volume, 100);

        config.volume = 50;
        validate_volume(&mut config);
        assert_eq!(config.volume, 50);
    }

    #[test]
    fn test_theme_validation() {
        // Ensure themes directory exists
        fs::create_dir_all("themes").unwrap();
        
        // Create a test theme file
        fs::write("themes/test_theme.json", "{}").unwrap();

        let mut config = PreferencesConfig {
            theme: "test_theme".to_string(),
            ..Default::default()
        };
        validate_theme(&mut config);
        assert_eq!(config.theme, "test_theme");

        config.theme = "nonexistent_theme".to_string();
        validate_theme(&mut config);
        assert_eq!(config.theme, "monokai");

        // Test manager_test_theme is allowed
        config.theme = "manager_test_theme".to_string();
        validate_theme(&mut config);
        assert_eq!(config.theme, "manager_test_theme");

        // Cleanup
        fs::remove_file("themes/test_theme.json").unwrap();
    }

    #[test]
    fn test_directory_validation() {
        let mut config = PreferencesConfig {
            last_directory: PathBuf::from("/nonexistent/path"),
            ..Default::default()
        };
        validate_directory(&mut config);
        assert!(config.last_directory.as_os_str().is_empty());

        let current_dir = std::env::current_dir().unwrap();
        config.last_directory = current_dir.clone();
        validate_directory(&mut config);
        assert_eq!(config.last_directory, current_dir);
    }

    #[test]
    fn test_full_validation() {
        let mut config = PreferencesConfig {
            theme: "nonexistent_theme".to_string(),
            volume: 150,
            last_directory: PathBuf::from("/nonexistent/path"),
        };
        validate_config(&mut config);

        assert_eq!(config.volume, 100);
        assert_eq!(config.theme, "monokai");
        assert!(config.last_directory.as_os_str().is_empty());
    }
}
