use std::path::Path;
use std::error::Error;
use crate::audio::AudioFormat;
use crate::audio::formats::{AudioDecoder, AudioReader};
use super::{flac, mp3, ogg, wav};

/// Enum to handle different decoder types
#[derive(Default)]
pub enum DecoderType {
    #[default]
    None,
    Flac(flac::FlacDecoder),
    Mp3(mp3::Mp3Decoder),
    Ogg(ogg::OggDecoder),
    Wav(wav::WavDecoder),
}

impl DecoderType {
    pub fn for_path(path: &Path) -> Self {
        match path.extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| ext.to_lowercase())
            .as_deref()
        {
            Some("flac") => Self::Flac(flac::FlacDecoder::new()),
            Some("mp3") => Self::Mp3(mp3::Mp3Decoder::new()),
            Some("ogg") => Self::Ogg(ogg::OggDecoder::new()),
            Some("wav") => Self::Wav(wav::WavDecoder::new()),
            _ => Self::None,
        }
    }
}

impl AudioDecoder for DecoderType {
    fn can_decode(&self, path: &Path) -> bool {
        match self {
            Self::None => false,
            Self::Flac(decoder) => decoder.can_decode(path),
            Self::Mp3(decoder) => decoder.can_decode(path),
            Self::Ogg(decoder) => decoder.can_decode(path),
            Self::Wav(decoder) => decoder.can_decode(path),
        }
    }

    fn probe_format(&self, path: &Path) -> Result<AudioFormat, Box<dyn Error>> {
        match self {
            Self::None => Err("No decoder available".into()),
            Self::Flac(decoder) => decoder.probe_format(path),
            Self::Mp3(decoder) => decoder.probe_format(path),
            Self::Ogg(decoder) => decoder.probe_format(path),
            Self::Wav(decoder) => decoder.probe_format(path),
        }
    }

    fn decode(&mut self, path: &Path) -> Result<AudioReader, Box<dyn Error>> {
        match self {
            Self::None => Err("No decoder available".into()),
            Self::Flac(decoder) => decoder.decode(path),
            Self::Mp3(decoder) => decoder.decode(path),
            Self::Ogg(decoder) => decoder.decode(path),
            Self::Wav(decoder) => decoder.decode(path),
        }
    }
}

/// Function to get a decoder for a specific file
pub fn get_decoder(path: &Path) -> DecoderType {
    DecoderType::for_path(path)
}
