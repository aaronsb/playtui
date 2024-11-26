use std::path::Path;
use std::fs::File;
use symphonia::core::probe::Hint;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::{MetadataOptions, Tag};
use crate::metadata::{Metadata, MetadataError};

pub struct TagExtractor;

impl TagExtractor {
    pub fn extract_metadata(path: &Path) -> Result<Metadata, MetadataError> {
        let file = File::open(path).map_err(MetadataError::IoError)?;
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let hint = Hint::new();
        let format_opts = FormatOptions::default();
        let metadata_opts = MetadataOptions::default();

        let mut probed = symphonia::default::get_probe()
            .format(&hint, mss, &format_opts, &metadata_opts)
            .map_err(|e| MetadataError::ParseError(e.to_string()))?;

        let mut metadata = Metadata::default();

        // Extract metadata from the format's current metadata
        if let Some(meta) = probed.format.metadata().current() {
            metadata = Self::extract_from_tags(meta.tags());
            
            // Add visual metadata if available
            if !meta.visuals().is_empty() {
                metadata.extra.insert("HAS_COVER_ART".to_string(), "true".to_string());
            }
        }

        Ok(metadata)
    }

    fn extract_from_tags(tags: &[Tag]) -> Metadata {
        let mut metadata = Metadata::default();

        // Helper function to find a tag value, handling multiple values
        let find_tag = |name: &str| -> Option<String> {
            tags.iter()
                .find(|t| t.key.eq_ignore_ascii_case(name))
                .map(|t| t.value.to_string())
        };

        // Extract standard tags with careful handling
        metadata.title = find_tag("TITLE");
        metadata.artist = find_tag("ARTIST").or_else(|| find_tag("PERFORMER"));
        metadata.album = find_tag("ALBUM");
        
        // Handle track number with potential disc number
        if let Some(track_str) = find_tag("TRACKNUMBER") {
            metadata.track = track_str
                .split('/')  // Handle "track/total" format
                .next()
                .and_then(|t| t.parse().ok());
        }

        // Handle various date formats
        metadata.year = find_tag("DATE")
            .or_else(|| find_tag("YEAR"))
            .and_then(|d| {
                // Try to extract year from various date formats
                d.split(|c| c == '-' || c == '/' || c == '.')
                    .next()
                    .and_then(|y| y.parse().ok())
            });

        metadata.genre = find_tag("GENRE");

        // Add all tags to extra, including alternate tag names
        for tag in tags {
            let key = tag.key.to_uppercase();
            let value = tag.value.to_string();
            
            // Store additional metadata that might be useful
            match key.as_str() {
                "ALBUMARTIST" | "ALBUM_ARTIST" => {
                    metadata.extra.insert("ALBUMARTIST".to_string(), value);
                }
                "DISCNUMBER" | "DISC" => {
                    metadata.extra.insert("DISCNUMBER".to_string(), value);
                }
                "COMPOSER" => {
                    metadata.extra.insert("COMPOSER".to_string(), value);
                }
                "COPYRIGHT" => {
                    metadata.extra.insert("COPYRIGHT".to_string(), value);
                }
                "ENCODED_BY" | "ENCODER" => {
                    metadata.extra.insert("ENCODER".to_string(), value);
                }
                _ => {
                    metadata.extra.insert(key, value);
                }
            }
        }

        metadata
    }
}
