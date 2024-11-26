use std::path::Path;
use metaflac::Tag;
use symphonia::core::probe::Hint;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use std::fs::File;

use crate::metadata::{Metadata, MetadataError, MetadataParser};
use super::matches_extension;

pub struct FlacMetadataParser;

impl FlacMetadataParser {
    pub fn new() -> Self {
        FlacMetadataParser
    }

    fn parse_vorbis_comments(&self, tag: &Tag) -> Metadata {
        let mut metadata = Metadata::default();

        if let Some(vc) = tag.vorbis_comments() {
            // Get standard tags
            if let Some(value) = vc.get("TITLE").and_then(|v| v.first()) {
                metadata.title = Some(value.to_string());
            }
            if let Some(value) = vc.get("ARTIST").and_then(|v| v.first()) {
                metadata.artist = Some(value.to_string());
            }
            if let Some(value) = vc.get("ALBUM").and_then(|v| v.first()) {
                metadata.album = Some(value.to_string());
            }
            if let Some(value) = vc.get("TRACKNUMBER").and_then(|v| v.first()) {
                metadata.track = value.parse().ok();
            }
            
            // Parse year from date
            if let Some(value) = vc.get("DATE").and_then(|v| v.first()) {
                metadata.year = value
                    .split('-')
                    .next()
                    .and_then(|y| y.parse().ok());
            }
            
            if let Some(value) = vc.get("GENRE").and_then(|v| v.first()) {
                metadata.genre = Some(value.to_string());
            }

            // Add all comments to extra
            for (key, values) in vc.comments.iter() {
                if let Some(value) = values.first() {
                    metadata.extra.insert(key.to_string(), value.to_string());
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
        
        // Calculate duration from total frames and sample rate
        let duration = params.n_frames
            .map(|frames| frames as f64 / params.sample_rate.unwrap_or(44100) as f64)
            .unwrap_or_default();
        
        let sample_rate = params.sample_rate.unwrap_or_default();
        let channels = params.channels.map(|ch| ch.count() as u8).unwrap_or_default();
        let bits_per_sample = params.bits_per_sample.unwrap_or(16) as u32;
        
        // Calculate bit rate from sample rate, channels, and bits per sample
        let bit_rate = (sample_rate * channels as u32 * bits_per_sample) / 1000;

        Ok((duration, sample_rate, channels, bit_rate))
    }
}

impl MetadataParser for FlacMetadataParser {
    fn parse(&self, path: &Path) -> Result<Metadata, MetadataError> {
        // Read FLAC metadata
        let tag = Tag::read_from_path(path)
            .map_err(|e| MetadataError::ParseError(e.to_string()))?;
        
        let mut metadata = self.parse_vorbis_comments(&tag);

        // Add audio properties
        if let Ok((duration, sample_rate, channels, bit_rate)) = self.parse_audio_properties(path) {
            metadata.duration = Some(duration);
            metadata.sample_rate = Some(sample_rate);
            metadata.channels = Some(channels);
            metadata.bit_rate = Some(bit_rate);
        }

        Ok(metadata)
    }

    fn supports_format(&self, path: &Path) -> bool {
        matches_extension(path, &["flac"])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_supports_format() {
        let parser = FlacMetadataParser::new();
        assert!(parser.supports_format(Path::new("test.flac")));
        assert!(!parser.supports_format(Path::new("test.mp3")));
        assert!(!parser.supports_format(Path::new("test.ogg")));
    }

    #[test]
    fn test_parse_flac() {
        let parser = FlacMetadataParser::new();
        let test_path = PathBuf::from("test/testaudio-short.flac");
        
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
