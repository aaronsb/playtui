use std::path::{Path, PathBuf};
use crate::metadata::MetadataParser;
use super::VorbisParser;

#[test]
fn test_supports_format() {
    let parser = VorbisParser::new();
    assert!(parser.supports_format(Path::new("test.ogg")));
    assert!(!parser.supports_format(Path::new("test.mp3")));
    assert!(!parser.supports_format(Path::new("test.flac")));
}

#[test]
fn test_parse_ogg() {
    let parser = VorbisParser::new();
    let test_path = PathBuf::from("test/testaudio-short.ogg");
    
    if test_path.exists() {
        let result = parser.parse(&test_path);
        assert!(result.is_ok());
        
        let metadata = result.unwrap();
        assert!(metadata.sample_rate.is_some());
        assert!(metadata.channels.is_some());
        assert!(metadata.duration.is_some());
    }
}
