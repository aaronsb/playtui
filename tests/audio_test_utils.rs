use std::path::Path;
use playtui::audio::formats::{
    AudioDecoder,
    flac::FlacDecoder,
    mp3::Mp3Decoder,
    ogg::OggDecoder,
    wav::WavDecoder
};

// Test data generators
pub fn generate_test_wav_data() -> Vec<u8> {
    // Basic WAV header + 1 second of silence at 44.1kHz, 16-bit stereo
    let mut data = Vec::new();
    
    // RIFF header
    data.extend(b"RIFF");                 // ChunkID
    data.extend(&[0x24, 0x00, 0x00, 0x00]); // ChunkSize (36 bytes)
    data.extend(b"WAVE");                 // Format
    
    // fmt subchunk
    data.extend(b"fmt ");                 // Subchunk1ID
    data.extend(&[0x10, 0x00, 0x00, 0x00]); // Subchunk1Size (16 bytes)
    data.extend(&[0x01, 0x00]);             // AudioFormat (1 = PCM)
    data.extend(&[0x02, 0x00]);             // NumChannels (2 = stereo)
    data.extend(&[0x44, 0xAC, 0x00, 0x00]); // SampleRate (44100)
    data.extend(&[0x10, 0xB1, 0x02, 0x00]); // ByteRate (176400)
    data.extend(&[0x04, 0x00]);             // BlockAlign (4)
    data.extend(&[0x10, 0x00]);             // BitsPerSample (16)
    
    // data subchunk
    data.extend(b"data");                 // Subchunk2ID
    data.extend(&[0x00, 0x00, 0x00, 0x00]); // Subchunk2Size (0 bytes of audio data)
    
    data
}

pub fn generate_test_flac_data() -> Vec<u8> {
    // Minimal FLAC header
    let mut data = Vec::new();
    data.extend(b"fLaC");  // FLAC stream marker
    // Add minimal FLAC metadata blocks here
    data
}

pub fn generate_test_mp3_data() -> Vec<u8> {
    // Minimal MP3 frame
    let mut data = Vec::new();
    // Add MP3 frame header and minimal data
    data.extend(&[0xFF, 0xFB]); // MPEG1 Layer 3 frame sync
    data
}

pub fn generate_test_ogg_data() -> Vec<u8> {
    // Minimal OGG header
    let mut data = Vec::new();
    data.extend(b"OggS");  // OGG stream marker
    // Add minimal OGG page headers here
    data
}

// Audio format validation
pub fn validate_wav_format(path: &Path) -> bool {
    let decoder = WavDecoder::new();
    decoder.can_decode(path)
}

pub fn validate_flac_format(path: &Path) -> bool {
    let decoder = FlacDecoder::new();
    decoder.can_decode(path)
}

pub fn validate_mp3_format(path: &Path) -> bool {
    let decoder = Mp3Decoder::new();
    decoder.can_decode(path)
}

pub fn validate_ogg_format(path: &Path) -> bool {
    let decoder = OggDecoder::new();
    decoder.can_decode(path)
}

// Audio format tests
#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    fn create_test_file(data: &[u8], extension: &str) -> std::path::PathBuf {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join(format!("test.{}", extension));
        let mut file = File::create(&file_path).unwrap();
        file.write_all(data).unwrap();
        file_path
    }

    #[test]
    fn test_wav_validation() {
        let data = generate_test_wav_data();
        let test_path = create_test_file(&data, "wav");
        assert!(validate_wav_format(&test_path), "Should validate WAV format");
        
        // Test invalid extension
        let test_path = create_test_file(&data, "mp3");
        assert!(!validate_wav_format(&test_path), "Should reject wrong extension");
    }

    #[test]
    fn test_flac_validation() {
        let data = generate_test_flac_data();
        let test_path = create_test_file(&data, "flac");
        assert!(validate_flac_format(&test_path), "Should validate FLAC format");
        
        // Test invalid extension
        let test_path = create_test_file(&data, "wav");
        assert!(!validate_flac_format(&test_path), "Should reject wrong extension");
    }

    #[test]
    fn test_mp3_validation() {
        let data = generate_test_mp3_data();
        let test_path = create_test_file(&data, "mp3");
        assert!(validate_mp3_format(&test_path), "Should validate MP3 format");
        
        // Test invalid extension
        let test_path = create_test_file(&data, "wav");
        assert!(!validate_mp3_format(&test_path), "Should reject wrong extension");
    }

    #[test]
    fn test_ogg_validation() {
        let data = generate_test_ogg_data();
        let test_path = create_test_file(&data, "ogg");
        assert!(validate_ogg_format(&test_path), "Should validate OGG format");
        
        // Test invalid extension
        let test_path = create_test_file(&data, "wav");
        assert!(!validate_ogg_format(&test_path), "Should reject wrong extension");
    }

    #[test]
    fn test_format_probing() {
        let wav_data = generate_test_wav_data();
        let flac_data = generate_test_flac_data();
        let mp3_data = generate_test_mp3_data();
        let ogg_data = generate_test_ogg_data();

        let wav_path = create_test_file(&wav_data, "wav");
        let flac_path = create_test_file(&flac_data, "flac");
        let mp3_path = create_test_file(&mp3_data, "mp3");
        let ogg_path = create_test_file(&ogg_data, "ogg");

        // Test format probing
        let wav_decoder = WavDecoder::new();
        let flac_decoder = FlacDecoder::new();
        let mp3_decoder = Mp3Decoder::new();
        let ogg_decoder = OggDecoder::new();

        assert!(wav_decoder.probe_format(&wav_path).is_ok(), "Should probe WAV format");
        assert!(flac_decoder.probe_format(&flac_path).is_ok(), "Should probe FLAC format");
        assert!(mp3_decoder.probe_format(&mp3_path).is_ok(), "Should probe MP3 format");
        assert!(ogg_decoder.probe_format(&ogg_path).is_ok(), "Should probe OGG format");
    }
}
