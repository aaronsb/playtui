use std::path::Path;
use std::sync::Arc;
use crate::metadata::{Metadata, MetadataError, MetadataParser};

/// Manages metadata parsing across different file formats
pub struct MetadataManager {
    parsers: Vec<Arc<dyn MetadataParser + Send + Sync>>,
}

impl MetadataManager {
    /// Create a new MetadataManager
    pub fn new() -> Self {
        MetadataManager {
            parsers: Vec::new(),
        }
    }

    /// Register a new metadata parser
    pub fn register_parser(&mut self, parser: Arc<dyn MetadataParser + Send + Sync>) {
        self.parsers.push(parser);
    }

    /// Parse metadata from a file
    pub fn parse_metadata(&self, path: &Path) -> Result<Metadata, MetadataError> {
        // Find a parser that supports this format
        for parser in &self.parsers {
            if parser.supports_format(path) {
                return parser.parse(path);
            }
        }
        
        Err(MetadataError::UnsupportedFormat)
    }

    /// Check if any registered parser supports the given format
    pub fn supports_format(&self, path: &Path) -> bool {
        self.parsers.iter().any(|parser| parser.supports_format(path))
    }
}

impl Default for MetadataManager {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    struct MockParser;

    impl MetadataParser for MockParser {
        fn parse(&self, _path: &Path) -> Result<Metadata, MetadataError> {
            Ok(Metadata {
                title: Some("Test Track".to_string()),
                artist: Some("Test Artist".to_string()),
                album: Some("Test Album".to_string()),
                track: Some(1),
                year: Some(2024),
                genre: Some("Test Genre".to_string()),
                duration: Some(180.0),
                sample_rate: Some(44100),
                channels: Some(2),
                bit_rate: Some(320),
                extra: HashMap::new(),
            })
        }

        fn supports_format(&self, path: &Path) -> bool {
            path.extension()
                .and_then(|ext| ext.to_str())
                .map(|ext| ext == "mp3")
                .unwrap_or(false)
        }
    }

    #[test]
    fn test_metadata_manager() {
        let mut manager = MetadataManager::new();
        let mock_parser = Arc::new(MockParser);
        manager.register_parser(mock_parser);

        let test_path = Path::new("test.mp3");
        assert!(manager.supports_format(test_path));
        
        let metadata = manager.parse_metadata(test_path).unwrap();
        assert_eq!(metadata.title.as_deref(), Some("Test Track"));
        assert_eq!(metadata.artist.as_deref(), Some("Test Artist"));
    }
}
