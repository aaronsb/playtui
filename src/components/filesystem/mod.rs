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
            selected_index: None,
        }
    }
}

impl FSState {
    pub fn new(path: PathBuf) -> Self {
        Self {
            current_dir: path,
            entries: Vec::new(),
            selected_index: None,
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
    }

    pub fn set_selected_index(&mut self, index: Option<usize>) {
        self.selected_index = index;
    }

    pub fn navigate_to(&mut self, path: PathBuf) {
        self.current_dir = path;
        self.selected_index = None;
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
