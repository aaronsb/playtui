use std::path::PathBuf;
use std::fs;
use std::io::{self, Read, Write};
use directories::ProjectDirs;
use log::{error, debug};
use super::config::PreferencesConfig;

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

/// Load preferences from the system configuration file
pub fn load_preferences() -> io::Result<PreferencesConfig> {
    let path = ensure_preferences_dir()?;
    
    // If file doesn't exist, return default config
    if !path.exists() {
        debug!("Preferences file not found, using defaults");
        return Ok(PreferencesConfig::default());
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
pub fn save_preferences(config: &PreferencesConfig) -> io::Result<()> {
    let path = ensure_preferences_dir()?;
    
    // Serialize to JSON
    let contents = serde_json::to_string_pretty(config).map_err(|e| {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
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
        save_preferences(&config).unwrap();
        
        // Load it back
        let loaded = load_preferences().unwrap();
        
        // Verify contents
        assert_eq!(loaded.theme, "save_load_test_theme");
        assert_eq!(loaded.volume, 75);
        assert_eq!(loaded.last_directory, PathBuf::from("/test/path"));

        // Clean up after test
        cleanup_preferences().unwrap();
    }
}
