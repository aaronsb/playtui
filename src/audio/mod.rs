use std::error::Error;
use std::time::Duration;

/// Represents the current state of audio playback
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlaybackState {
    Playing,
    Paused,
    Stopped,
}

/// Represents audio format metadata
#[derive(Debug, Clone)]
pub struct AudioFormat {
    pub channels: u16,
    pub sample_rate: u32,
    pub bits_per_sample: u16,
}

/// Core trait for audio playback functionality
pub trait AudioPlayer {
    /// Initialize the audio player
    fn initialize(&mut self) -> Result<(), Box<dyn Error>>;
    
    /// Load an audio file for playback
    fn load(&mut self, path: &str) -> Result<(), Box<dyn Error>>;
    
    /// Start or resume playback
    fn play(&mut self) -> Result<(), Box<dyn Error>>;
    
    /// Pause playback
    fn pause(&mut self) -> Result<(), Box<dyn Error>>;
    
    /// Stop playback
    fn stop(&mut self) -> Result<(), Box<dyn Error>>;
    
    /// Get current playback position
    fn position(&self) -> Duration;
    
    /// Get total duration of the current track
    fn duration(&self) -> Option<Duration>;
    
    /// Set playback position
    fn seek(&mut self, position: Duration) -> Result<(), Box<dyn Error>>;
    
    /// Get current playback state
    fn state(&self) -> PlaybackState;
    
    /// Get current audio format
    fn format(&self) -> Option<AudioFormat>;
}

/// Trait for managing audio streams
pub trait AudioStream {
    /// Open an audio stream with given format
    fn open(&mut self, format: AudioFormat) -> Result<(), Box<dyn Error>>;
    
    /// Write audio data to the stream
    fn write(&mut self, data: &[u8]) -> Result<usize, Box<dyn Error>>;
    
    /// Close the audio stream
    fn close(&mut self) -> Result<(), Box<dyn Error>>;
}

pub mod player;
pub mod stream;
pub mod formats;
