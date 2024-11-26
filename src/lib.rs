pub mod app;
pub mod ui;
pub mod audio;
pub mod components;
pub mod metadata;

use std::sync::Arc;
use std::time::Duration;

use metadata::{
    MetadataManager,
    FileMetadataCache,
    formats::{Id3Parser, VorbisParser, FlacMetadataParser},
};

/// Creates a new configured MetadataManager with all supported format parsers
pub fn create_metadata_manager() -> MetadataManager {
    let mut manager = MetadataManager::new();
    
    // Register parsers for supported formats
    manager.register_parser(Arc::new(Id3Parser::new()));
    manager.register_parser(Arc::new(VorbisParser::new()));
    manager.register_parser(Arc::new(FlacMetadataParser::new()));
    
    manager
}

/// Creates a new metadata cache with the specified maximum age
pub fn create_metadata_cache(max_age: Duration) -> FileMetadataCache {
    FileMetadataCache::new(max_age)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_metadata_manager_creation() {
        let manager = create_metadata_manager();
        
        // Test format support
        assert!(manager.supports_format(Path::new("test.mp3")));
        assert!(manager.supports_format(Path::new("test.ogg")));
        assert!(manager.supports_format(Path::new("test.flac")));
        assert!(!manager.supports_format(Path::new("test.unknown")));
    }

    #[test]
    fn test_metadata_cache_creation() {
        let cache = create_metadata_cache(Duration::from_secs(3600));
        assert!(cache.is_empty());
    }
}
