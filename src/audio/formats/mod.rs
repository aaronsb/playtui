use std::error::Error;
use std::path::Path;
use crate::audio::AudioFormat;

mod audio_reader;
mod decoder_factory;
#[cfg(test)]
mod tests;

// Re-export format-specific modules
pub mod flac;
pub mod mp3;
pub mod ogg;
pub mod wav;

// Re-export key types
pub use audio_reader::AudioReader;
pub use decoder_factory::{DecoderType, get_decoder};

/// Trait for audio format decoders
pub trait AudioDecoder {
    /// Check if the given file is in this format
    fn can_decode(&self, path: &Path) -> bool;
    
    /// Get the audio format details without fully loading the file
    fn probe_format(&self, path: &Path) -> Result<AudioFormat, Box<dyn Error>>;
    
    /// Read audio data from the file
    fn decode(&mut self, path: &Path) -> Result<AudioReader, Box<dyn Error>>;
}
