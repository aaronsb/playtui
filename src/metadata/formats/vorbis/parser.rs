use std::path::Path;
use crate::metadata::{Metadata, MetadataError, MetadataParser};
use super::{matches_extension, tag_extractor::TagExtractor, audio_properties::AudioPropertiesExtractor};

pub struct VorbisParser;

impl VorbisParser {
    pub fn new() -> Self {
        VorbisParser
    }
}

impl MetadataParser for VorbisParser {
    fn parse(&self, path: &Path) -> Result<Metadata, MetadataError> {
        // First get the metadata from tags
        let mut metadata = TagExtractor::extract_metadata(path)?;

        // Then add audio properties
        if let Ok((duration, sample_rate, channels, bit_rate)) = AudioPropertiesExtractor::parse_properties(path) {
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
