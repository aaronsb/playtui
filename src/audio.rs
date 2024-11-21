use anyhow::Result;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::{fs::File, io::BufReader, path::Path, sync::Arc};

pub struct AudioPlayer {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Option<Arc<Sink>>,
}

impl AudioPlayer {
    pub fn new() -> Result<Self> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        
        Ok(Self {
            _stream: stream,
            stream_handle,
            sink: None,
        })
    }

    pub fn play<P: AsRef<Path>>(&mut self, path: P) -> Result<()> {
        // Create a new sink
        let sink = Sink::try_new(&self.stream_handle)?;
        
        // Open the audio file
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        
        // Decode the audio file
        let source = Decoder::new(reader)?;
        
        // Play the audio
        sink.append(source);
        self.sink = Some(Arc::new(sink));
        
        Ok(())
    }

    pub fn pause(&self) {
        if let Some(sink) = &self.sink {
            sink.pause();
        }
    }

    pub fn resume(&self) {
        if let Some(sink) = &self.sink {
            sink.play();
        }
    }

    pub fn stop(&mut self) {
        if let Some(sink) = &self.sink {
            sink.stop();
        }
        self.sink = None;
    }

    pub fn set_volume(&self, volume: f32) {
        if let Some(sink) = &self.sink {
            sink.set_volume(volume);
        }
    }

    pub fn is_playing(&self) -> bool {
        self.sink.as_ref().map_or(false, |sink| !sink.empty())
    }
}

impl Drop for AudioPlayer {
    fn drop(&mut self) {
        self.stop();
    }
}
