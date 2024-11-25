use std::error::Error;
use std::path::Path;
use crate::audio::AudioFormat;

/// Trait for audio format decoders
pub trait AudioDecoder {
    /// Check if the given file is in this format
    fn can_decode(&self, path: &Path) -> bool;
    
    /// Get the audio format details without fully loading the file
    fn probe_format(&self, path: &Path) -> Result<AudioFormat, Box<dyn Error>>;
    
    /// Read audio data from the file
    fn decode(&mut self, path: &Path) -> Result<AudioReader, Box<dyn Error>>;
}

/// Represents a reader for decoded audio data
pub struct AudioReader {
    pub format: AudioFormat,
    pub total_samples: u64,
    buffer: Vec<f32>, // Normalized float samples
    position: usize,
}

impl AudioReader {
    pub fn new(format: AudioFormat, total_samples: u64) -> Self {
        Self {
            format,
            total_samples,
            buffer: Vec::new(),
            position: 0,
        }
    }

    /// Read the next chunk of samples into the provided buffer
    pub fn read(&mut self, buffer: &mut [f32]) -> Result<usize, Box<dyn Error>> {
        // TODO: Implement actual reading from the decoded audio data
        Ok(0)
    }

    /// Seek to a specific sample position
    pub fn seek(&mut self, sample_pos: u64) -> Result<(), Box<dyn Error>> {
        if sample_pos >= self.total_samples {
            return Err("Seek position out of bounds".into());
        }
        // TODO: Implement actual seeking in the decoded audio data
        Ok(())
    }

    /// Get current position in samples
    pub fn position(&self) -> u64 {
        self.position as u64
    }
}

// Re-export format-specific modules
pub mod flac;
pub mod mp3;
pub mod ogg;
pub mod wav;

// Create an enum to handle different decoder types
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

// Function to get a decoder for a specific file
pub fn get_decoder(path: &Path) -> DecoderType {
    DecoderType::for_path(path)
}
