use std::path::PathBuf;

/// Represents a file system entry (file or directory)
#[derive(Clone, Debug)]
pub struct FSEntry {
    path: PathBuf,
    name: String,
    is_dir: bool,
}

impl FSEntry {
    pub fn new(path: PathBuf) -> Self {
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();
        let is_dir = path.is_dir();
        Self { path, name, is_dir }
    }

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn is_dir(&self) -> bool {
        self.is_dir
    }
}

/// State for filesystem navigation
#[derive(Clone, Debug)]
pub struct FSState {
    current_dir: PathBuf,
    entries: Vec<FSEntry>,
    selected_index: Option<usize>,
}

impl Default for FSState {
    fn default() -> Self {
        Self {
            current_dir: PathBuf::from("."),
            entries: Vec::new(),
            selected_index: Some(0),  // Initialize with first item selected
        }
    }
}

impl FSState {
    pub fn new(path: PathBuf) -> Self {
        // Attempt to canonicalize the path, fall back to original if it fails
        let current_dir = path.canonicalize().unwrap_or_else(|_| path.clone());
        Self {
            current_dir,
            entries: Vec::new(),
            selected_index: Some(0),  // Initialize with first item selected
        }
    }

    pub fn current_dir(&self) -> &PathBuf {
        &self.current_dir
    }

    pub fn entries(&self) -> &[FSEntry] {
        &self.entries
    }

    pub fn selected_index(&self) -> Option<usize> {
        self.selected_index
    }

    pub fn set_entries(&mut self, entries: Vec<FSEntry>) {
        self.entries = entries;
        // Only set initial selection if there isn't one already
        if self.selected_index.is_none() {
            self.selected_index = if self.entries.is_empty() { None } else { Some(0) };
        }
        // Ensure selection is still valid after changing entries
        if let Some(index) = self.selected_index {
            if index >= self.entries.len() {
                self.selected_index = if self.entries.is_empty() { None } else { Some(0) };
            }
        }
    }

    pub fn set_selected_index(&mut self, index: Option<usize>) {
        // Ensure the index is valid
        self.selected_index = index.filter(|&i| i < self.entries.len());
    }

    pub fn navigate_to(&mut self, path: PathBuf) {
        // Attempt to canonicalize the path, fall back to original if it fails
        self.current_dir = path.canonicalize().unwrap_or(path);
        // Reset selection when changing directories
        self.selected_index = Some(0);
        self.entries.clear();
    }
}

/// Actions specific to filesystem operations
#[derive(Clone, Debug)]
pub enum FSAction {
    NavigateToParent,
    NavigateToSelected,
    Select(usize),
    Refresh,
}

mod navigator;
pub use navigator::FSNavigator;
