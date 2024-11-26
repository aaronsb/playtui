use std::time::Duration;
use std::path::PathBuf;

use super::*;
use crate::{create_metadata_manager, create_metadata_cache};

#[test]
fn test_complete_metadata_system() {
    // Create manager and cache
    let manager = create_metadata_manager();
    let mut cache = create_metadata_cache(Duration::from_secs(3600));

    // Test paths for different formats
    let test_files = [
        "test/testaudio-short.mp3",
        "test/testaudio-short.ogg",
        "test/testaudio-short.flac",
    ];

    for file_path in test_files.iter() {
        let path = PathBuf::from(file_path);
        
        if path.exists() {
            // Test format support
            assert!(manager.supports_format(&path));

            // Parse metadata
            let result = manager.parse_metadata(&path);
            assert!(result.is_ok(), "Failed to parse metadata for {}", file_path);

            let metadata = result.unwrap();

            // Verify essential audio properties are present
            assert!(metadata.sample_rate.is_some(), "Missing sample rate for {}", file_path);
            assert!(metadata.channels.is_some(), "Missing channels for {}", file_path);
            assert!(metadata.duration.is_some(), "Missing duration for {}", file_path);

            // Test caching
            cache.store(&path, metadata.clone());
            let cached = cache.get(&path);
            assert!(cached.is_some(), "Failed to retrieve from cache for {}", file_path);
            assert_eq!(
                cached.unwrap().sample_rate,
                metadata.sample_rate,
                "Cache mismatch for {}",
                file_path
            );
        }
    }

    // Test cache cleanup
    cache.cleanup();
    assert!(!cache.is_empty(), "Cache should not be empty after cleanup");

    // Test cache expiration
    let expired_cache = create_metadata_cache(Duration::from_nanos(1));
    std::thread::sleep(Duration::from_millis(1));
    assert!(expired_cache.get(&PathBuf::from("test/testaudio-short.mp3")).is_none());
}

#[test]
fn test_metadata_fields() {
    let manager = create_metadata_manager();
    let test_files = [
        "test/testaudio-short.mp3",
        "test/testaudio-short.ogg",
        "test/testaudio-short.flac",
    ];

    for file_path in test_files.iter() {
        let path = PathBuf::from(file_path);
        
        if path.exists() {
            if let Ok(metadata) = manager.parse_metadata(&path) {
                // Print metadata for inspection
                println!("Metadata for {}:", file_path);
                println!("  Sample Rate: {:?} Hz", metadata.sample_rate);
                println!("  Channels: {:?}", metadata.channels);
                println!("  Duration: {:?} seconds", metadata.duration);
                println!("  Bit Rate: {:?} kbps", metadata.bit_rate);
                println!("  Title: {:?}", metadata.title);
                println!("  Artist: {:?}", metadata.artist);
                println!("  Album: {:?}", metadata.album);
                println!("  Extra tags: {} found", metadata.extra.len());
                println!();

                // Basic assertions
                assert!(metadata.sample_rate.unwrap_or(0) > 0);
                assert!(metadata.channels.unwrap_or(0) > 0);
                assert!(metadata.duration.unwrap_or(0.0) > 0.0);
            }
        }
    }
}
