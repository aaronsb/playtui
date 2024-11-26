pub mod app;
pub mod audio;
pub mod components;
pub mod events;
pub mod metadata;
pub mod state;
pub mod theme;
pub mod ui;

use crate::metadata::{MetadataManager, FileMetadataCache};
use std::time::Duration;

pub fn create_metadata_manager() -> MetadataManager {
    MetadataManager::new()
}

pub fn create_metadata_cache(max_age: Duration) -> FileMetadataCache {
    FileMetadataCache::new(max_age)
}
