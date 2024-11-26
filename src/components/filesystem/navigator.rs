use std::path::PathBuf;
use anyhow::{Result, Context};
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
        
        // Always try to add parent directory entry if it exists
        if let Some(parent) = self.state.current_dir().parent() {
            entries.push(FSEntry::new(parent.to_path_buf()));
        }

        // Attempt to scan directory entries
        match std::fs::read_dir(self.state.current_dir()) {
            Ok(dir_entries) => {
                // Process readable entries
                for entry in dir_entries {
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
            }
            Err(e) => {
                // If we can't read the directory but have a parent,
                // keep the parent entry to allow navigation back up
                if entries.is_empty() {
                    // If we have no entries at all (including parent), propagate the error
                    return Err(e).context("Failed to read directory contents");
                }
            }
        }

        self.state.set_entries(entries);
        Ok(())
    }

    /// Act phase: Handle navigation actions
    pub fn handle_action(&mut self, action: FSAction) -> Result<()> {
        match action {
            FSAction::NavigateToParent => {
                // Always attempt to navigate to parent if it exists
                if let Some(parent) = self.state.current_dir().parent() {
                    self.state.navigate_to(parent.to_path_buf());
                    // Even if scanning fails, we've already updated the path
                    let _ = self.scan_current_dir();
                }
            }
            FSAction::NavigateToSelected => {
                if let Some(index) = self.state.selected_index() {
                    if let Some(entry) = self.state.entries().get(index) {
                        if entry.is_dir() {
                            // Store the current directory in case we need to revert
                            let previous_dir = self.state.current_dir().clone();
                            
                            // Attempt to navigate to the selected directory
                            self.state.navigate_to(entry.path().clone());
                            if let Err(e) = self.scan_current_dir() {
                                // If we can't access the directory, revert to previous
                                self.state.navigate_to(previous_dir);
                                let _ = self.scan_current_dir();
                                return Err(e).context("Failed to access selected directory");
                            }
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
                let _ = self.scan_current_dir();
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
