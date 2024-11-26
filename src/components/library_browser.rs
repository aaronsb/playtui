use ratatui::{
    prelude::*,
    widgets::{List, ListItem, ListState},
};
use super::{Component, ComponentState, create_block};
use crate::events::{Event, Action, KeyEvent, NavigationEvent};
use crate::components::filesystem::{FSNavigator, FSAction};
use crate::theme::Theme;
use std::{path::PathBuf, cell::RefCell};

#[derive(Clone)]
pub struct LibraryBrowser {
    state: ComponentState,
    fs_navigator: RefCell<FSNavigator>,
    list_state: RefCell<ListState>,
}

impl Component for LibraryBrowser {
    fn new() -> Self {
        // Start in the current directory
        let fs_navigator = FSNavigator::new(PathBuf::from("."));
        let mut list_state = ListState::default();
        list_state.select(Some(0)); // Initialize with first item selected
        
        let browser = Self {
            state: ComponentState::default(),
            fs_navigator: RefCell::new(fs_navigator),
            list_state: RefCell::new(list_state),
        };
        
        // Initial directory scan
        if let Err(e) = browser.fs_navigator.borrow_mut().scan_current_dir() {
            eprintln!("Error scanning directory: {}", e);
        }
        
        browser
    }

    fn render(&self, frame: &mut Frame, area: Rect, focused: bool, theme: &Theme) {
        let block = create_block("Library Browser", focused, theme);
        
        // Get all the data we need in one borrow
        let navigator = self.fs_navigator.borrow();
        let entries = navigator.state().entries().to_vec();
        let selected = navigator.state().selected_index();
        drop(navigator);
        
        // Create list items
        let items: Vec<ListItem> = entries
            .iter()
            .map(|entry| {
                let prefix = if entry.is_dir() { "📁 " } else { "📄 " };
                ListItem::new(format!("{}{}", prefix, entry.name()))
                    .style(if focused {
                        theme.get_style("list_item")
                    } else {
                        theme.get_style("list_item_unfocused")
                    })
            })
            .collect();

        // Update list state
        let mut list_state = self.list_state.borrow_mut();
        list_state.select(selected);

        let list = List::new(items)
            .block(block)
            .highlight_style(
                if focused {
                    theme.get_style("list_selected")
                        .add_modifier(Modifier::BOLD)
                } else {
                    theme.get_style("list_selected_unfocused")
                }
            )
            .highlight_symbol("▶ "); // More visible cursor

        frame.render_stateful_widget(list, area, &mut list_state);
    }

    fn update(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::NavigateUp => {
                // Get all the data we need in one borrow
                let mut navigator = self.fs_navigator.borrow_mut();
                let current_index = navigator.state().selected_index().unwrap_or(0);
                let new_index = if current_index > 0 { current_index - 1 } else { 0 };
                
                if let Err(e) = navigator.handle_action(FSAction::Select(new_index)) {
                    eprintln!("Error navigating: {}", e);
                }
            }
            Action::NavigateDown => {
                // Get all the data we need in one borrow
                let mut navigator = self.fs_navigator.borrow_mut();
                let current_index = navigator.state().selected_index().unwrap_or(0);
                let max_index = navigator.state().entries().len().saturating_sub(1);
                let new_index = if current_index < max_index { current_index + 1 } else { max_index };
                
                if let Err(e) = navigator.handle_action(FSAction::Select(new_index)) {
                    eprintln!("Error navigating: {}", e);
                }
            }
            Action::NavigateRight | Action::Select => {
                if let Err(e) = self.fs_navigator.borrow_mut().handle_action(FSAction::NavigateToSelected) {
                    eprintln!("Error selecting entry: {}", e);
                }
            }
            Action::NavigateLeft | Action::Back => {
                if let Err(e) = self.fs_navigator.borrow_mut().handle_action(FSAction::NavigateToParent) {
                    eprintln!("Error navigating to parent: {}", e);
                }
            }
            _ => return None,
        }
        Some(Action::Refresh)
    }

    fn focused(&self) -> bool {
        self.state.focused
    }

    fn set_focused(&mut self, focused: bool) {
        self.state.focused = focused;
    }

    fn handle_event(&mut self, event: Event) -> Option<Action> {
        match event {
            Event::Key(key_event) if self.focused() => match key_event {
                KeyEvent::Up => Some(Action::NavigateUp),
                KeyEvent::Down => Some(Action::NavigateDown),
                KeyEvent::Right | KeyEvent::Enter => Some(Action::Select),
                KeyEvent::Left | KeyEvent::Escape => Some(Action::Back),
                _ => None,
            },
            Event::Navigation(nav_event) => match nav_event {
                NavigationEvent::Up => Some(Action::NavigateUp),
                NavigationEvent::Down => Some(Action::NavigateDown),
                NavigationEvent::Right => Some(Action::NavigateRight),
                NavigationEvent::Left => Some(Action::NavigateLeft),
            },
            _ => None,
        }
    }
}
