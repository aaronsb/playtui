use anyhow::Result;
use id3::{Tag, TagLike};
use std::path::Path;
use crate::app::Track;
use std::fs::File;
use std::io::BufReader;
use rodio::Decoder;
use rodio::Source;

pub fn scan_directory<P: AsRef<Path>>(path: P) -> Result<Vec<Track>> {
    let mut tracks = Vec::new();
    
    // Only scan the current directory, not recursively
    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries {
            if let Ok(entry) = entry {
                let path = entry.path();
                if path.is_file() {
                    if let Some(extension) = path.extension() {
                        if let Some(ext) = extension.to_str() {
                            if matches!(ext.to_lowercase().as_str(), "mp3" | "ogg" | "flac" | "wav") {
                                if let Ok(track) = create_track_from_path(&path) {
                                    tracks.push(track);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    // Sort tracks by title
    tracks.sort_by(|a, b| {
        let a_title = a.title.as_deref().unwrap_or("");
        let b_title = b.title.as_deref().unwrap_or("");
        a_title.cmp(b_title)
    });

    Ok(tracks)
}

fn create_track_from_path(path: &Path) -> Result<Track> {
    let mut title = None;
    let mut artist = None;
    let mut duration = None;

    // Try to read ID3 tags for MP3 files
    if let Some(ext) = path.extension() {
        if ext == "mp3" {
            if let Ok(tag) = Tag::read_from_path(path) {
                title = tag.title().map(String::from);
                artist = tag.artist().map(String::from);
            }
        }
    }

    // If no title found, use filename
    if title.is_none() {
        if let Some(file_name) = path.file_name().and_then(|n| n.to_str()) {
            let clean_name = clean_filename(file_name);
            
            // Try to split into title and artist
            if let Some((title_part, artist_part)) = split_title_artist(&clean_name) {
                title = Some(title_part);
                artist = Some(artist_part);
            } else {
                title = Some(clean_name);
                artist = Some("Unknown Artist".to_string());
            }
        }
    }

    // Get audio duration using rodio
    if let Ok(file) = File::open(path) {
        if let Ok(decoder) = Decoder::new(BufReader::new(file)) {
            let total_duration = decoder.total_duration();
            if let Some(total) = total_duration {
                duration = Some(total.as_secs());
            }
        }
    }

    Ok(Track {
        path: path.to_path_buf(),
        title,
        artist,
        duration,
    })
}

fn clean_filename(filename: &str) -> String {
    // Remove file extension
    let name = filename
        .trim_end_matches(".flac")
        .trim_end_matches(".mp3")
        .trim_end_matches(".ogg")
        .trim_end_matches(".wav");

    // Remove leading numbers and common separators
    let name = if let Some(pos) = name.find(char::is_alphabetic) {
        &name[pos..]
    } else {
        name
    };

    // Clean up the text
    name.trim()
        .replace('_', " ")
        .replace("  ", " ")
        .trim()
        .to_string()
}

fn split_title_artist(name: &str) -> Option<(String, String)> {
    // Try different separators
    let separators = [" - ", "-", "–", "—"];
    
    for separator in separators {
        if let Some(idx) = name.find(separator) {
            let (title_part, artist_part) = name.split_at(idx);
            let artist_part = artist_part.trim_start_matches(separator);
            
            if !title_part.is_empty() && !artist_part.is_empty() {
                return Some((
                    title_part.trim().to_string(),
                    artist_part.trim().to_string(),
                ));
            }
        }
    }
    
    None
}
