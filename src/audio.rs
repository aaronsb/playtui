use anyhow::Result;
use rodio::{Decoder, OutputStream, OutputStreamHandle, Sink};
use std::{fs::File, io::BufReader, path::Path, sync::Arc, time::Duration};

pub struct AudioPlayer {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Option<Arc<Sink>>,
    start_time: Option<std::time::Instant>,
    paused_position: Duration,
}

impl AudioPlayer {
    pub fn new() -> Result<Self> {
        let (stream, stream_handle) = OutputStream::try_default()?;
        
        Ok(Self {
            _stream: stream,
            stream_handle,
            sink: None,
            start_time: None,
            paused_position: Duration::from_secs(0),
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
        self.start_time = Some(std::time::Instant::now());
        self.paused_position = Duration::from_secs(0);
        
        Ok(())
    }

    pub fn pause(&mut self) {
        if let Some(sink) = &self.sink {
            sink.pause();
            if let Some(start) = self.start_time {
                self.paused_position += start.elapsed();
                self.start_time = None;
            }
        }
    }

    pub fn resume(&mut self) {
        if let Some(sink) = &self.sink {
            sink.play();
            self.start_time = Some(std::time::Instant::now());
        }
    }

    pub fn stop(&mut self) {
        if let Some(sink) = &self.sink {
            sink.stop();
        }
        self.sink = None;
        self.start_time = None;
        self.paused_position = Duration::from_secs(0);
    }

    pub fn set_volume(&self, volume: f32) {
        if let Some(sink) = &self.sink {
            sink.set_volume(volume);
        }
    }

    pub fn is_playing(&self) -> bool {
        self.sink.as_ref().map_or(false, |sink| !sink.empty())
    }

    pub fn position(&self) -> u64 {
        if let Some(start) = self.start_time {
            self.paused_position.as_secs() + start.elapsed().as_secs()
        } else {
            self.paused_position.as_secs()
        }
    }
}

impl Drop for AudioPlayer {
    fn drop(&mut self) {
        self.stop();
    }
}
