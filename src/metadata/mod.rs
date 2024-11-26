use std::path::Path;
use std::collections::HashMap;

/// Represents the metadata of an audio file
#[derive(Debug, Clone, Default)]
pub struct Metadata {
    /// Title of the track
    pub title: Option<String>,
    /// Artist name
    pub artist: Option<String>,
    /// Album name
    pub album: Option<String>,
    /// Track number in album
    pub track: Option<u32>,
    /// Year of release
    pub year: Option<u32>,
    /// Genre
    pub genre: Option<String>,
    /// Duration in seconds
    pub duration: Option<f64>,
    /// Sample rate in Hz
    pub sample_rate: Option<u32>,
    /// Number of channels
    pub channels: Option<u8>,
    /// Bit rate in kbps
    pub bit_rate: Option<u32>,
    /// Additional format-specific metadata
    pub extra: HashMap<String, String>,
}

/// Error types for metadata operations
#[derive(Debug)]
pub enum MetadataError {
    /// File could not be read
    IoError(std::io::Error),
    /// File format not supported
    UnsupportedFormat,
    /// Error parsing metadata
    ParseError(String),
    /// Required metadata field missing
    MissingField(String),
}

/// Trait for metadata parsing
pub trait MetadataParser {
    /// Parse metadata from a file path
    fn parse(&self, path: &Path) -> Result<Metadata, MetadataError>;
    
    /// Check if this parser supports the given file format
    fn supports_format(&self, path: &Path) -> bool;
}

/// Trait for metadata caching
pub trait MetadataCache {
    /// Get metadata from cache if available
    fn get(&self, path: &Path) -> Option<Metadata>;
    
    /// Store metadata in cache
    fn store(&mut self, path: &Path, metadata: Metadata);
    
    /// Remove metadata from cache
    fn remove(&mut self, path: &Path);
    
    /// Clear the entire cache
    fn clear(&mut self);
}

pub mod parser;
pub mod cache;
pub mod formats;

// Re-export commonly used items
pub use self::parser::MetadataManager;
pub use self::cache::FileMetadataCache;
