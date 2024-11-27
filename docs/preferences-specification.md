# Preferences System Specification

## Overview
The preferences system provides persistent storage of user settings across sessions, with graceful fallback behavior for missing or corrupt configuration files.

## File Location
- Uses the `directories` crate to determine system-appropriate config location
- Linux: `~/.config/playtui/preferences.json`

## Configuration Structure
```json
{
  "theme": "monokai",     // Must match an existing theme name
  "volume": 50,           // Integer 0-100
  "last_directory": ""    // String path, empty for default
}
```

## Core Components

### 1. PreferencesManager
```rust
pub struct PreferencesManager {
    config: PreferencesConfig,
    file_path: PathBuf,
    dirty: bool,
}

impl PreferencesManager {
    /// Creates new instance with system-appropriate path
    pub fn new() -> Result<Self>;
    
    /// Loads existing config or creates default
    pub fn load() -> Result<Self>;
    
    /// Saves current config if dirty
    pub fn save(&mut self) -> Result<()>;
    
    /// Updates last directory and marks dirty
    pub fn update_last_directory(&mut self, path: PathBuf);
    
    /// Updates volume and marks dirty
    pub fn update_volume(&mut self, volume: u8);
    
    /// Updates theme and marks dirty
    pub fn update_theme(&mut self, theme: String);
}
```

### 2. PreferencesConfig
```rust
#[derive(Serialize, Deserialize)]
pub struct PreferencesConfig {
    pub theme: String,
    pub volume: u8,
    pub last_directory: PathBuf,
}

impl Default for PreferencesConfig {
    fn default() -> Self {
        Self {
            theme: "monokai".to_string(),
            volume: 50,
            last_directory: PathBuf::new(),
        }
    }
}
```

## Error Handling

### File Access Errors
1. Missing Configuration
   - Create default configuration
   - Attempt to save to disk
   - Log creation attempt result

2. Corrupt Configuration
   - Load default configuration in memory
   - Log warning for user visibility
   - Do not overwrite corrupt file
   - User must manually fix or delete

3. Unwritable Location
   - Use in-memory configuration
   - Log error for user visibility
   - Retry saves periodically

## Integration with Event System

### 1. Initialization Flow
```rust
// During app startup
1. PreferencesManager::load()
2. Apply theme from preferences
3. Set volume control level
4. Navigate to last directory
```

### 2. Event Handling
```rust
// In event processing
match event {
    // Volume changes
    Event::VolumeChanged(level) => {
        preferences.update_volume(level);
        Action::UpdateVolume(level)
    },
    
    // Directory navigation
    Event::DirectoryChanged(path) => {
        preferences.update_last_directory(path);
        Action::NavigateDirectory(path)
    },
    
    // Theme changes
    Event::ThemeChanged(name) => {
        preferences.update_theme(name);
        Action::ApplyTheme(name)
    },
}
```

### 3. Graceful Shutdown
```rust
// During app shutdown
1. Capture final directory location
2. Save preferences to disk
3. Log any save errors
```

## Theme System Integration

### 1. Theme Loading
- Validate theme name exists in themes directory
- Fall back to "default" theme if specified theme missing
- Apply theme settings to UI components

### 2. Theme Persistence
- Save theme name on changes
- Validate theme exists before saving
- Maintain last working theme if new theme invalid

## Volume Integration

### 1. Volume State
- Load volume level at startup
- Apply to audio system
- Update preferences on volume changes

### 2. Volume Constraints
- Ensure volume between 0-100
- Normalize out-of-bounds values
- Log normalization events

## Directory Persistence

### 1. Directory State
- Load last directory at startup
- Validate directory exists and is accessible
- Fall back to home directory if invalid

### 2. Directory Updates
- Track directory navigation
- Update preferences on changes
- Validate paths before saving

## Testing Strategy

### 1. Unit Tests
- Configuration serialization/deserialization
- Default value generation
- Path validation and normalization

### 2. Integration Tests
- File system interactions
- Error handling scenarios
- Event system integration

### 3. System Tests
- Cross-platform path handling
- Permission scenarios
- Corrupt file recovery
