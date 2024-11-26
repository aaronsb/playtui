use std::path::Path;
use id3::{Tag, TagLike};
use symphonia::core::probe::Hint;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use std::fs::File;

use crate::metadata::{Metadata, MetadataError, MetadataParser};
use super::matches_extension;

pub struct Id3Parser;

impl Id3Parser {
    pub fn new() -> Self {
        Id3Parser
    }

    fn parse_id3_tag(&self, tag: &Tag) -> Metadata {
        let mut metadata = Metadata::default();

        metadata.title = tag.title().map(String::from);
        metadata.artist = tag.artist().map(String::from);
        metadata.album = tag.album().map(String::from);
        metadata.track = tag.track().map(|t| t as u32);
        metadata.year = tag.year().map(|y| y as u32);
        metadata.genre = tag.genre().map(String::from);

        // Add any additional ID3 frames to extra
        for frame in tag.frames() {
            metadata.extra.insert(
                frame.id().to_string(),
                frame.content().to_string()
            );
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
        
        // For MP3, estimate bitrate from file size and duration
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
}

impl MetadataParser for Id3Parser {
    fn parse(&self, path: &Path) -> Result<Metadata, MetadataError> {
        // Read ID3 tags
        let tag = Tag::read_from_path(path)
            .map_err(|e| MetadataError::ParseError(e.to_string()))?;
        
        let mut metadata = self.parse_id3_tag(&tag);

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
        matches_extension(path, &["mp3"])
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn test_supports_format() {
        let parser = Id3Parser::new();
        assert!(parser.supports_format(Path::new("test.mp3")));
        assert!(!parser.supports_format(Path::new("test.ogg")));
        assert!(!parser.supports_format(Path::new("test.flac")));
    }

    #[test]
    fn test_parse_mp3() {
        let parser = Id3Parser::new();
        let test_path = PathBuf::from("test/testaudio-short.mp3");
        
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
