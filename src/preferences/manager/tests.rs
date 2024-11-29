#[cfg(test)]
use super::*;
use std::fs;
use std::thread;
use std::os::unix::fs::PermissionsExt;
use crate::preferences::persistence::get_preferences_path;
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
    for _ in 0..SaveHandler::MAX_SAVE_FAILURES {
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
    assert!(!manager.save_handler.is_fs_writable());
    assert_eq!(manager.save_handler.save_failures(), SaveHandler::MAX_SAVE_FAILURES);

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
    for _ in 0..SaveHandler::MAX_SAVE_FAILURES {
        manager.update_theme("test".to_string());
        let _ = manager.save();
    }

    // Verify immediate retry is skipped
    assert!(!manager.save_handler.is_fs_writable());
    let save_result = manager.save();
    assert!(save_result.is_ok()); // Save should "succeed" but actually be skipped

    // Wait for retry interval
    thread::sleep(SaveHandler::RETRY_INTERVAL);

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
