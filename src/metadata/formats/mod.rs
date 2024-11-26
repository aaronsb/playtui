mod id3;
mod vorbis;
mod flac_meta;

pub use self::id3::Id3Parser;
pub use self::vorbis::VorbisParser;
pub use self::flac_meta::FlacMetadataParser;

use std::path::Path;

/// Get the file extension from a path
pub(crate) fn get_extension(path: &Path) -> Option<String> {
    path.extension()
        .and_then(|ext| ext.to_str())
        .map(|s| s.to_lowercase())
}

/// Check if a file extension matches any of the given extensions
pub(crate) fn matches_extension(path: &Path, extensions: &[&str]) -> bool {
    get_extension(path)
        .map(|ext| extensions.iter().any(|&e| e == ext))
        .unwrap_or(false)
}
