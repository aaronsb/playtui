use std::path::Path;
use std::fs::File;
use symphonia::core::probe::Hint;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::meta::Tag;

use crate::metadata::{Metadata, MetadataError, MetadataParser};
use super::matches_extension;

pub struct VorbisParser;

impl VorbisParser {
    pub fn new() -> Self {
        VorbisParser
    }

    fn extract_metadata_from_tags(&self, tags: &[Tag]) -> Metadata {
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

    fn parse_audio_properties(&self, path: &Path) -> Result<(f64, u32, u8, u32), MetadataError> {
        let file = File::open(path).map_err(MetadataError::IoError)?;
        let mss = MediaSourceStream::new(Box::new(file), Default::default());

        let hint = Hint::new();
        let format_opts = FormatOptions::default();
        let metadata_opts = MetadataOptions::default();

        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &format_opts, &metadata_opts)
            .map_err(|e| MetadataError::ParseError(e.to_string()))?;

        let track = probed
            .format
            .default_track()
            .ok_or_else(|| MetadataError::ParseError("No default track found".to_string()))?;

        let params = &track.codec_params;
        
        // Get duration more accurately using time_base if available
        let duration = if let Some(n_frames) = params.n_frames {
            if let Some(tb) = params.time_base {
                (n_frames as f64 * tb.numer as f64) / tb.denom as f64
            } else {
                n_frames as f64 / params.sample_rate.unwrap_or(44100) as f64
            }
        } else {
            0.0
        };
        
        let sample_rate = params.sample_rate.unwrap_or_default();
        let channels = params.channels.map(|ch| ch.count() as u8).unwrap_or_default();
        
        // Calculate average bit rate from file size and duration
        let bit_rate = if duration > 0.0 {
            let file_size = std::fs::metadata(path)
                .map(|m| m.len())
                .unwrap_or(0) as f64;
            ((file_size * 8.0) / (duration * 1000.0)) as u32 // Convert to kbps
        } else {
            0
        };

        Ok((duration, sample_rate, channels, bit_rate))
    }

    fn extract_metadata(&self, path: &Path) -> Result<Metadata, MetadataError> {
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
            metadata = self.extract_metadata_from_tags(meta.tags());
            
            // Add visual metadata if available
            if !meta.visuals().is_empty() {
                metadata.extra.insert("HAS_COVER_ART".to_string(), "true".to_string());
            }
        }

        Ok(metadata)
    }
}

impl MetadataParser for VorbisParser {
    fn parse(&self, path: &Path) -> Result<Metadata, MetadataError> {
        // First get the metadata from tags
        let mut metadata = self.extract_metadata(path)?;

        // Then add audio properties
        if let Ok((duration, sample_rate, channels, bit_rate)) = self.parse_audio_properties(path) {
            metadata.duration = Some(duration);
            metadata.sample_rate = Some(sample_rate);
            metadata.channels = Some(channels);
            metadata.bit_rate = Some(bit_rate);
        }

        Ok(metadata)
    }

    fn supports_format(&self, path: &Path) -> bool {
        matches_extension(path, &["ogg"])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_supports_format() {
        let parser = VorbisParser::new();
        assert!(parser.supports_format(Path::new("test.ogg")));
        assert!(!parser.supports_format(Path::new("test.mp3")));
        assert!(!parser.supports_format(Path::new("test.flac")));
    }

    #[test]
    fn test_parse_ogg() {
        let parser = VorbisParser::new();
        let test_path = PathBuf::from("test/testaudio-short.ogg");
        
        if test_path.exists() {
            let result = parser.parse(&test_path);
            assert!(result.is_ok());
            
            let metadata = result.unwrap();
            assert!(metadata.sample_rate.is_some());
            assert!(metadata.channels.is_some());
            assert!(metadata.duration.is_some());
        }
    }
}
