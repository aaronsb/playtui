mod config;
mod persistence;
mod validation;
mod manager;

// Public exports
pub use config::PreferencesConfig;
pub use manager::PreferencesManager;
pub use persistence::get_preferences_path;
