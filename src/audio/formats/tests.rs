use std::path::Path;
use super::*;

#[test]
fn test_decoder_factory() {
    let path = Path::new("test.mp3");
    let decoder = get_decoder(path);
    
    match decoder {
        DecoderType::Mp3(_) => assert!(true),
        _ => assert!(false, "Expected MP3 decoder for .mp3 file"),
    }
}

#[test]
fn test_audio_reader_creation() {
    let format = AudioFormat {
        channels: 2,
        sample_rate: 44100,
        bits_per_sample: 16,
    };
    
    let reader = AudioReader::new(format, 1000);
    assert_eq!(reader.total_samples, 1000);
    assert_eq!(reader.position(), 0);
}
