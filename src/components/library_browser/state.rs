use std::{path::PathBuf, cell::RefCell};
use crate::components::filesystem::{FSNavigator, FSAction};
use crate::components::ComponentState;

#[derive(Clone)]
pub struct LibraryBrowserState {
    pub(crate) state: ComponentState,
    pub(crate) fs_navigator: RefCell<FSNavigator>,
}

impl LibraryBrowserState {
    pub fn new() -> Self {
        // Start in the current directory
        let fs_navigator = FSNavigator::new(PathBuf::from("."));
        let browser_state = Self {
            state: ComponentState::default(),
            fs_navigator: RefCell::new(fs_navigator),
        };
        
        // Initial directory scan
        if let Err(e) = browser_state.fs_navigator.borrow_mut().scan_current_dir() {
            eprintln!("Error scanning directory: {}", e);
        }
        
        browser_state
    }

    pub fn focused(&self) -> bool {
        self.state.focused
    }

    pub fn set_focused(&mut self, focused: bool) {
        self.state.focused = focused;
    }

    pub fn navigate_up(&mut self) -> Result<(), String> {
        let mut navigator = self.fs_navigator.borrow_mut();
        let current_index = navigator.state().selected_index().unwrap_or(0);
        let new_index = if current_index > 0 { current_index - 1 } else { 0 };
        
        navigator.handle_action(FSAction::Select(new_index))
            .map_err(|e| e.to_string())
    }

    pub fn navigate_down(&mut self) -> Result<(), String> {
        let mut navigator = self.fs_navigator.borrow_mut();
        let current_index = navigator.state().selected_index().unwrap_or(0);
        let max_index = navigator.state().entries().len().saturating_sub(1);
        let new_index = if current_index < max_index { current_index + 1 } else { max_index };
        
        navigator.handle_action(FSAction::Select(new_index))
            .map_err(|e| e.to_string())
    }

    pub fn navigate_to_selected(&mut self) -> Result<(), String> {
        self.fs_navigator.borrow_mut()
            .handle_action(FSAction::NavigateToSelected)
            .map_err(|e| e.to_string())
    }

    pub fn navigate_to_parent(&mut self) -> Result<(), String> {
        self.fs_navigator.borrow_mut()
            .handle_action(FSAction::NavigateToParent)
            .map_err(|e| e.to_string())
    }

    pub fn select_index(&mut self, index: usize) -> Result<(), String> {
        self.fs_navigator.borrow_mut()
            .handle_action(FSAction::Select(index))
            .map_err(|e| e.to_string())
    }

    pub fn get_entries(&self) -> Vec<crate::components::filesystem::FSEntry> {
        self.fs_navigator.borrow().state().entries().to_vec()
    }

    pub fn get_selected_index(&self) -> Option<usize> {
        self.fs_navigator.borrow().state().selected_index()
    }
}
