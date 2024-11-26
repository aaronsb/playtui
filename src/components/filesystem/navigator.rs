use std::path::PathBuf;
use anyhow::Result;
use super::{FSEntry, FSState, FSAction};

/// Handles filesystem navigation and entry listing
#[derive(Clone)]
pub struct FSNavigator {
    state: FSState,
}

impl FSNavigator {
    pub fn new(initial_path: PathBuf) -> Self {
        Self {
            state: FSState::new(initial_path),
        }
    }

    /// Observe phase: Scan current directory and update entries
    pub fn scan_current_dir(&mut self) -> Result<()> {
        let mut entries = Vec::new();
        
        // Add parent directory entry if not at root
        if let Some(parent) = self.state.current_dir().parent() {
            entries.push(FSEntry::new(parent.to_path_buf()));
        }

        // Scan directory entries
        for entry in std::fs::read_dir(self.state.current_dir())? {
            if let Ok(entry) = entry {
                entries.push(FSEntry::new(entry.path()));
            }
        }

        // Sort entries: directories first, then files, alphabetically within each group
        entries.sort_by(|a, b| {
            match (a.is_dir(), b.is_dir()) {
                (true, false) => std::cmp::Ordering::Less,
                (false, true) => std::cmp::Ordering::Greater,
                _ => a.name().cmp(b.name()),
            }
        });

        self.state.set_entries(entries);
        Ok(())
    }

    /// Act phase: Handle navigation actions
    pub fn handle_action(&mut self, action: FSAction) -> Result<()> {
        match action {
            FSAction::NavigateToParent => {
                if let Some(parent) = self.state.current_dir().parent() {
                    self.state.navigate_to(parent.to_path_buf());
                    self.scan_current_dir()?;
                }
            }
            FSAction::NavigateToSelected => {
                if let Some(index) = self.state.selected_index() {
                    if let Some(entry) = self.state.entries().get(index) {
                        if entry.is_dir() {
                            self.state.navigate_to(entry.path().clone());
                            self.scan_current_dir()?;
                        }
                    }
                }
            }
            FSAction::Select(index) => {
                if index < self.state.entries().len() {
                    self.state.set_selected_index(Some(index));
                }
            }
            FSAction::Refresh => {
                self.scan_current_dir()?;
            }
        }
        Ok(())
    }

    /// Get current state
    pub fn state(&self) -> &FSState {
        &self.state
    }

    /// Get mutable state reference
    pub fn state_mut(&mut self) -> &mut FSState {
        &mut self.state
    }
}
