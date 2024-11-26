use std::path::{Path, PathBuf};
use std::collections::HashMap;
use std::time::{SystemTime, Duration};
use crate::metadata::{Metadata, MetadataCache};

/// Cached metadata entry with timestamp
struct CacheEntry {
    metadata: Metadata,
    timestamp: SystemTime,
}

/// File-based metadata cache implementation
pub struct FileMetadataCache {
    entries: HashMap<PathBuf, CacheEntry>,
    max_age: Duration,
}

impl FileMetadataCache {
    /// Create a new cache with the specified maximum age for entries
    pub fn new(max_age: Duration) -> Self {
        FileMetadataCache {
            entries: HashMap::new(),
            max_age,
        }
    }

    /// Create a new cache with default maximum age (1 hour)
    pub fn default() -> Self {
        Self::new(Duration::from_secs(3600))
    }

    /// Remove expired entries from the cache
    pub fn cleanup(&mut self) {
        let now = SystemTime::now();
        self.entries.retain(|_, entry| {
            now.duration_since(entry.timestamp)
                .map(|age| age <= self.max_age)
                .unwrap_or(false)
        });
    }

    /// Get the number of entries in the cache
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Check if the cache is empty
    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }
}

impl MetadataCache for FileMetadataCache {
    fn get(&self, path: &Path) -> Option<Metadata> {
        let now = SystemTime::now();
        
        self.entries.get(path).and_then(|entry| {
            // Check if entry is still valid
            now.duration_since(entry.timestamp)
                .map(|age| if age <= self.max_age {
                    Some(entry.metadata.clone())
                } else {
                    None
                })
                .unwrap_or(None)
        })
    }

    fn store(&mut self, path: &Path, metadata: Metadata) {
        self.entries.insert(
            path.to_path_buf(),
            CacheEntry {
                metadata,
                timestamp: SystemTime::now(),
            },
        );
    }

    fn remove(&mut self, path: &Path) {
        self.entries.remove(path);
    }

    fn clear(&mut self) {
        self.entries.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;

    #[test]
    fn test_cache_operations() {
        let mut cache = FileMetadataCache::new(Duration::from_secs(1));
        let path = Path::new("test.mp3");
        let metadata = Metadata::default();

        // Test store and get
        cache.store(path, metadata.clone());
        assert!(cache.get(path).is_some());
        
        // Test expiration
        thread::sleep(Duration::from_secs(2));
        assert!(cache.get(path).is_none());

        // Test remove
        cache.store(path, metadata.clone());
        assert!(!cache.is_empty());
        cache.remove(path);
        assert!(cache.is_empty());

        // Test clear
        cache.store(path, metadata);
        assert_eq!(cache.len(), 1);
        cache.clear();
        assert_eq!(cache.len(), 0);
    }

    #[test]
    fn test_cleanup() {
        let mut cache = FileMetadataCache::new(Duration::from_millis(100));
        let path1 = Path::new("test1.mp3");
        let path2 = Path::new("test2.mp3");
        
        cache.store(path1, Metadata::default());
        thread::sleep(Duration::from_millis(200));
        cache.store(path2, Metadata::default());
        
        cache.cleanup();
        assert!(cache.get(path1).is_none());
        assert!(cache.get(path2).is_some());
    }
}
