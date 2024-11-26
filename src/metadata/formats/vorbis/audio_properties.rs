use std::path::Path;
use std::fs::File;
use symphonia::core::probe::Hint;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use crate::metadata::MetadataError;

pub struct AudioPropertiesExtractor;

impl AudioPropertiesExtractor {
    pub fn parse_properties(path: &Path) -> Result<(f64, u32, u8, u32), MetadataError> {
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
        
        // Get duration more accurately using time_base if available
        let duration = if let Some(n_frames) = params.n_frames {
            if let Some(tb) = params.time_base {
                (n_frames as f64 * tb.numer as f64) / tb.denom as f64
            } else {
                n_frames as f64 / params.sample_rate.unwrap_or(44100) as f64
            }
        } else {
            0.0
        };
        
        let sample_rate = params.sample_rate.unwrap_or_default();
        let channels = params.channels.map(|ch| ch.count() as u8).unwrap_or_default();
        
        // Calculate average bit rate from file size and duration
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
