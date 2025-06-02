use std::fs::File;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::thread;

use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use symphonia::default::get_probe;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{FromSample, SizedSample};

pub struct AudioPlayer {
    state: Arc<Mutex<PlayerState>>,
    control_tx: Option<std::sync::mpsc::Sender<PlayerCommand>>,
    playback_thread: Option<thread::JoinHandle<()>>,
}

#[derive(Clone)]
struct PlayerState {
    is_playing: bool,
    is_paused: bool,
    volume: f32,
    current_position: Duration,
    total_duration: Duration,
}

enum PlayerCommand {
    Play(String),
    Pause,
    Resume,
    Stop,
    SetVolume(f32),
}

impl AudioPlayer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            state: Arc::new(Mutex::new(PlayerState {
                is_playing: false,
                is_paused: false,
                volume: 0.5,
                current_position: Duration::from_secs(0),
                total_duration: Duration::from_secs(0),
            })),
            control_tx: None,
            playback_thread: None,
        })
    }

    pub fn play(&mut self, path: &Path) -> Result<(), Box<dyn std::error::Error>> {
        // Stop current playback
        self.stop();

        // Get track duration first
        let duration = get_track_duration(path)?;
        
        // Update state
        {
            let mut state = self.state.lock().unwrap();
            state.total_duration = duration;
            state.current_position = Duration::from_secs(0);
            state.is_playing = true;
            state.is_paused = false;
        }

        // Create control channel
        let (tx, rx) = std::sync::mpsc::channel();
        self.control_tx = Some(tx);

        // Start playback thread
        let state = Arc::clone(&self.state);
        let path = path.to_string_lossy().to_string();
        
        self.playback_thread = Some(thread::spawn(move || {
            if let Err(e) = playback_thread(path, state, rx) {
                eprintln!("Playback error: {}", e);
            }
        }));

        Ok(())
    }

    pub fn pause(&mut self) {
        if let Some(tx) = &self.control_tx {
            let _ = tx.send(PlayerCommand::Pause);
        }
    }

    pub fn resume(&mut self) {
        if let Some(tx) = &self.control_tx {
            let _ = tx.send(PlayerCommand::Resume);
        }
    }

    pub fn stop(&mut self) {
        if let Some(tx) = &self.control_tx {
            let _ = tx.send(PlayerCommand::Stop);
        }
        
        if let Some(thread) = self.playback_thread.take() {
            let _ = thread.join();
        }
        
        self.control_tx = None;
        
        let mut state = self.state.lock().unwrap();
        state.is_playing = false;
        state.is_paused = false;
        state.current_position = Duration::from_secs(0);
    }

    pub fn set_volume(&mut self, volume: f32) {
        let mut state = self.state.lock().unwrap();
        state.volume = volume;
        
        if let Some(tx) = &self.control_tx {
            let _ = tx.send(PlayerCommand::SetVolume(volume));
        }
    }

    pub fn is_playing(&self) -> bool {
        let state = self.state.lock().unwrap();
        state.is_playing && !state.is_paused
    }

    pub fn has_finished(&self) -> bool {
        let state = self.state.lock().unwrap();
        !state.is_playing
    }

    pub fn get_position(&self) -> Duration {
        let state = self.state.lock().unwrap();
        state.current_position
    }

    pub fn get_duration(&self) -> Duration {
        let state = self.state.lock().unwrap();
        state.total_duration
    }
}

fn get_track_duration(path: &Path) -> Result<Duration, Box<dyn std::error::Error>> {
    let file = File::open(path)?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    
    let hint = Hint::new();
    let format_opts = FormatOptions::default();
    let metadata_opts = MetadataOptions::default();
    
    let probed = get_probe().format(&hint, mss, &format_opts, &metadata_opts)?;
    let format = probed.format;
    
    if let Some(track) = format.default_track() {
        if let Some(timebase) = track.codec_params.time_base {
            if let Some(duration) = track.codec_params.n_frames {
                let secs = timebase.calc_time(duration).seconds as u64;
                return Ok(Duration::from_secs(secs));
            }
        }
    }
    
    Ok(Duration::from_secs(0))
}

fn playback_thread(
    path: String,
    state: Arc<Mutex<PlayerState>>,
    rx: std::sync::mpsc::Receiver<PlayerCommand>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Open the audio file
    let file = File::open(&path)?;
    let mss = MediaSourceStream::new(Box::new(file), Default::default());
    
    let hint = Hint::new();
    let format_opts = FormatOptions::default();
    let metadata_opts = MetadataOptions::default();
    let decoder_opts = DecoderOptions::default();
    
    let probed = get_probe().format(&hint, mss, &format_opts, &metadata_opts)?;
    let format = probed.format;
    
    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .ok_or("No supported audio tracks")?;
    
    let decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &decoder_opts)?;
    
    let track_id = track.id;
    
    // Setup audio output
    let host = cpal::default_host();
    let device = host.default_output_device()
        .ok_or("No output device available")?;
    
    let config = device.default_output_config()?;
    
    // Create audio output stream
    let err_fn = |err| eprintln!("Audio stream error: {}", err);
    
    match config.sample_format() {
        cpal::SampleFormat::F32 => run_output::<f32>(
            &device,
            &config.into(),
            state,
            rx,
            format,
            decoder,
            track_id,
            err_fn,
        )?,
        cpal::SampleFormat::I16 => run_output::<i16>(
            &device,
            &config.into(),
            state,
            rx,
            format,
            decoder,
            track_id,
            err_fn,
        )?,
        cpal::SampleFormat::U16 => run_output::<u16>(
            &device,
            &config.into(),
            state,
            rx,
            format,
            decoder,
            track_id,
            err_fn,
        )?,
        _ => return Err("Unsupported sample format".into()),
    }
    
    Ok(())
}

fn run_output<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    state: Arc<Mutex<PlayerState>>,
    rx: std::sync::mpsc::Receiver<PlayerCommand>,
    mut format: Box<dyn symphonia::core::formats::FormatReader>,
    mut decoder: Box<dyn symphonia::core::codecs::Decoder>,
    track_id: u32,
    err_fn: impl FnMut(cpal::StreamError) + Send + 'static,
) -> Result<(), Box<dyn std::error::Error>>
where
    T: SizedSample + FromSample<f32>,
{
    let output_sample_rate = config.sample_rate.0;
    let channels = config.channels as usize;
    
    // Get the input sample rate from the decoder
    let input_sample_rate = decoder.codec_params().sample_rate.unwrap_or(44100);
    
    
    // Larger buffer for smoother playback
    let (sample_tx, sample_rx) = std::sync::mpsc::sync_channel::<Vec<f32>>(1024);
    
    // Audio output stream
    let state_clone = Arc::clone(&state);
    let mut sample_buffer: Vec<f32> = Vec::new();
    let mut buffer_pos = 0;
    let mut samples_since_last_data = 0;
    let sample_rate = config.sample_rate.0 as usize;
    
    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            let (volume, is_playing) = {
                let mut state = state_clone.lock().unwrap();
                let vol = state.volume;
                let playing = state.is_playing && !state.is_paused;
                
                // Check if we should mark as finished due to no data
                if playing && samples_since_last_data > sample_rate {
                    state.is_playing = false;
                    (vol, false)
                } else {
                    (vol, playing)
                }
            };
            
            if !is_playing {
                // Not playing, fill with silence
                for sample in data.iter_mut() {
                    *sample = T::from_sample(0.0);
                }
                return;
            }
            
            let mut output_pos = 0;
            
            while output_pos < data.len() {
                // Need more samples?
                if buffer_pos >= sample_buffer.len() {
                    // Try to get more samples
                    if let Ok(new_samples) = sample_rx.try_recv() {
                        sample_buffer = new_samples;
                        buffer_pos = 0;
                        samples_since_last_data = 0;
                    } else {
                        // No samples available
                        // Only mark as truly finished if we haven't received data for a while
                        samples_since_last_data += data.len() - output_pos;
                        
                        // Increment the counter for silence detection
                        // The state check happens at the beginning of the callback
                        
                        // Fill rest with silence
                        for i in output_pos..data.len() {
                            data[i] = T::from_sample(0.0);
                        }
                        break;
                    }
                }
                
                // Copy samples from buffer to output
                while buffer_pos < sample_buffer.len() && output_pos < data.len() {
                    data[output_pos] = T::from_sample(sample_buffer[buffer_pos] * volume);
                    buffer_pos += 1;
                    output_pos += 1;
                }
            }
        },
        err_fn,
        None,
    )?;
    
    stream.play()?;
    
    // Pre-fill buffer before starting
    let mut initial_packets = 0;
    while initial_packets < 10 {
        match format.next_packet() {
            Ok(packet) => {
                if packet.track_id() != track_id {
                    continue;
                }
                
                match decoder.decode(&packet) {
                    Ok(decoded) => {
                        let mut samples = convert_samples(decoded, channels);
                        
                        if input_sample_rate != output_sample_rate {
                            samples = resample_audio(
                                samples, 
                                channels,
                                input_sample_rate,
                                output_sample_rate
                            );
                        }
                        
                        let _ = sample_tx.send(samples);
                        initial_packets += 1;
                    }
                    Err(_) => continue,
                }
            }
            Err(_) => break,
        }
    }
    
    // Decode and send samples
    let start_time = std::time::Instant::now();
    let mut is_paused = false;
    let mut pause_duration = Duration::from_secs(0);
    let mut pause_start = None;
    let mut decoding_finished = false;
    
    loop {
        // Check for control commands
        if let Ok(cmd) = rx.try_recv() {
            match cmd {
                PlayerCommand::Stop => break,
                PlayerCommand::Pause => {
                    let mut state = state.lock().unwrap();
                    state.is_paused = true;
                    is_paused = true;
                    pause_start = Some(std::time::Instant::now());
                }
                PlayerCommand::Resume => {
                    let mut state = state.lock().unwrap();
                    state.is_paused = false;
                    is_paused = false;
                    if let Some(start) = pause_start {
                        pause_duration += start.elapsed();
                        pause_start = None;
                    }
                }
                PlayerCommand::SetVolume(vol) => {
                    let mut state = state.lock().unwrap();
                    state.volume = vol;
                }
                _ => {}
            }
        }
        
        if !is_paused && !decoding_finished {
            // Update position
            {
                let mut state = state.lock().unwrap();
                let elapsed = start_time.elapsed() - pause_duration;
                state.current_position = elapsed;
            }
            
            // Decode next packet
            match format.next_packet() {
                Ok(packet) => {
                    if packet.track_id() != track_id {
                        continue;
                    }
                    
                    match decoder.decode(&packet) {
                        Ok(decoded) => {
                            let mut samples = convert_samples(decoded, channels);
                            
                            // Simple resampling if needed
                            if input_sample_rate != output_sample_rate {
                                samples = resample_audio(
                                    samples, 
                                    channels,
                                    input_sample_rate,
                                    output_sample_rate
                                );
                            }
                            
                            // Use try_send with retry to avoid blocking when buffer is full
                            loop {
                                match sample_tx.try_send(samples.clone()) {
                                    Ok(_) => break,
                                    Err(std::sync::mpsc::TrySendError::Full(_)) => {
                                        // Buffer full, wait a bit
                                        thread::sleep(Duration::from_millis(5));
                                        
                                        // Check for stop command while waiting
                                        if let Ok(PlayerCommand::Stop) = rx.try_recv() {
                                            decoding_finished = true;
                                            break;
                                        }
                                    }
                                    Err(std::sync::mpsc::TrySendError::Disconnected(_)) => {
                                        // Channel closed, mark as finished
                                        decoding_finished = true;
                                        break;
                                    }
                                }
                            }
                        }
                        Err(Error::DecodeError(_)) => continue,
                        Err(err) => return Err(err.into()),
                    }
                }
                Err(Error::ResetRequired) => {
                    decoder.reset();
                }
                Err(Error::IoError(err)) if err.kind() == std::io::ErrorKind::UnexpectedEof => {
                    decoding_finished = true;
                }
                Err(err) => return Err(err.into()),
            }
        } else if decoding_finished {
            // All packets decoded, wait for audio to finish playing
            // Check if audio is still playing by monitoring the state
            let is_still_playing = {
                let state = state.lock().unwrap();
                state.is_playing && !state.is_paused
            };
            
            if !is_still_playing {
                break;
            }
            
            // Also check for stop command
            if let Ok(PlayerCommand::Stop) = rx.try_recv() {
                break;
            }
            
            thread::sleep(Duration::from_millis(50));
        } else {
            thread::sleep(Duration::from_millis(10));
        }
    }
    
    // Wait a bit for any remaining buffered audio to play
    thread::sleep(Duration::from_millis(500));
    
    // Ensure the state is marked as not playing
    {
        let mut state = state.lock().unwrap();
        state.is_playing = false;
    }
    
    // Keep the stream alive until this function returns
    // The stream will be dropped here, stopping playback
    drop(stream);
    
    Ok(())
}

fn convert_samples(audio_buf: AudioBufferRef<'_>, output_channels: usize) -> Vec<f32> {
    let mut samples = Vec::new();
    let input_channels = audio_buf.spec().channels.count();
    
    match audio_buf {
        AudioBufferRef::F32(buf) => {
            let frames = buf.frames();
            for i in 0..frames {
                if input_channels == 1 && output_channels == 2 {
                    // Mono to stereo: duplicate the sample
                    let sample = buf.chan(0)[i];
                    samples.push(sample);
                    samples.push(sample);
                } else if input_channels == 2 && output_channels == 1 {
                    // Stereo to mono: average the channels
                    let left = buf.chan(0)[i];
                    let right = buf.chan(1)[i];
                    samples.push((left + right) / 2.0);
                } else if input_channels == 2 && output_channels == 2 {
                    // Stereo to stereo: direct copy
                    samples.push(buf.chan(0)[i]);
                    samples.push(buf.chan(1)[i]);
                } else {
                    // Default: just use first channel
                    samples.push(buf.chan(0)[i]);
                    for _ in 1..output_channels {
                        samples.push(0.0);
                    }
                }
            }
        }
        AudioBufferRef::S16(buf) => {
            let frames = buf.frames();
            let scale = 1.0 / i16::MAX as f32;
            
            for i in 0..frames {
                if input_channels == 1 && output_channels == 2 {
                    // Mono to stereo: duplicate the sample
                    let sample = buf.chan(0)[i] as f32 * scale;
                    samples.push(sample);
                    samples.push(sample);
                } else if input_channels == 2 && output_channels == 1 {
                    // Stereo to mono: average the channels
                    let left = buf.chan(0)[i] as f32 * scale;
                    let right = buf.chan(1)[i] as f32 * scale;
                    samples.push((left + right) / 2.0);
                } else if input_channels == 2 && output_channels == 2 {
                    // Stereo to stereo: direct copy
                    samples.push(buf.chan(0)[i] as f32 * scale);
                    samples.push(buf.chan(1)[i] as f32 * scale);
                } else {
                    // Default: just use first channel
                    samples.push(buf.chan(0)[i] as f32 * scale);
                    for _ in 1..output_channels {
                        samples.push(0.0);
                    }
                }
            }
        }
        AudioBufferRef::S32(buf) => {
            let frames = buf.frames();
            let scale = 1.0 / i32::MAX as f32;
            
            for i in 0..frames {
                if input_channels == 1 && output_channels == 2 {
                    // Mono to stereo: duplicate the sample
                    let sample = buf.chan(0)[i] as f32 * scale;
                    samples.push(sample);
                    samples.push(sample);
                } else if input_channels == 2 && output_channels == 1 {
                    // Stereo to mono: average the channels
                    let left = buf.chan(0)[i] as f32 * scale;
                    let right = buf.chan(1)[i] as f32 * scale;
                    samples.push((left + right) / 2.0);
                } else if input_channels == 2 && output_channels == 2 {
                    // Stereo to stereo: direct copy
                    samples.push(buf.chan(0)[i] as f32 * scale);
                    samples.push(buf.chan(1)[i] as f32 * scale);
                } else {
                    // Default: just use first channel
                    samples.push(buf.chan(0)[i] as f32 * scale);
                    for _ in 1..output_channels {
                        samples.push(0.0);
                    }
                }
            }
        }
        AudioBufferRef::U8(buf) => {
            let frames = buf.frames();
            let scale = 1.0 / 128.0;
            let offset = 128.0;
            
            for i in 0..frames {
                if input_channels == 1 && output_channels == 2 {
                    // Mono to stereo: duplicate the sample
                    let sample = (buf.chan(0)[i] as f32 - offset) * scale;
                    samples.push(sample);
                    samples.push(sample);
                } else if input_channels == 2 && output_channels == 1 {
                    // Stereo to mono: average the channels
                    let left = (buf.chan(0)[i] as f32 - offset) * scale;
                    let right = (buf.chan(1)[i] as f32 - offset) * scale;
                    samples.push((left + right) / 2.0);
                } else if input_channels == 2 && output_channels == 2 {
                    // Stereo to stereo: direct copy
                    samples.push((buf.chan(0)[i] as f32 - offset) * scale);
                    samples.push((buf.chan(1)[i] as f32 - offset) * scale);
                } else {
                    // Default: just use first channel
                    samples.push((buf.chan(0)[i] as f32 - offset) * scale);
                    for _ in 1..output_channels {
                        samples.push(0.0);
                    }
                }
            }
        }
        _ => {
            // Should not happen with symphonia
            // Don't spam the console
            let frames = audio_buf.frames();
            for _ in 0..frames {
                for _ in 0..output_channels {
                    samples.push(0.0);
                }
            }
        }
    }
    
    samples
}

fn resample_audio(
    input: Vec<f32>, 
    channels: usize,
    input_rate: u32,
    output_rate: u32
) -> Vec<f32> {
    if input_rate == output_rate {
        return input;
    }
    
    let ratio = input_rate as f64 / output_rate as f64;
    let input_frames = input.len() / channels;
    let output_frames = (input_frames as f64 / ratio) as usize;
    let mut output = Vec::with_capacity(output_frames * channels);
    
    // Simple linear interpolation resampling
    for out_frame in 0..output_frames {
        let in_frame_f = out_frame as f64 * ratio;
        let in_frame = in_frame_f as usize;
        let fract = in_frame_f - in_frame as f64;
        
        for ch in 0..channels {
            let idx1 = in_frame * channels + ch;
            let idx2 = ((in_frame + 1) * channels + ch).min(input.len() - 1);
            
            if idx1 < input.len() && idx2 < input.len() {
                let sample1 = input[idx1];
                let sample2 = input[idx2];
                let sample = sample1 + (sample2 - sample1) * fract as f32;
                output.push(sample);
            } else if idx1 < input.len() {
                output.push(input[idx1]);
            } else {
                output.push(0.0);
            }
        }
    }
    
    output
}