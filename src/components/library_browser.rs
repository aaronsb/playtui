use ratatui::{
    prelude::*,
    widgets::{List, ListItem},
};
use super::{Component, ComponentState, create_block};
use crate::events::{Event, Action, KeyEvent};
use crate::components::filesystem::{FSNavigator, FSAction};
use crate::theme::Theme;
use std::path::PathBuf;

#[derive(Clone)]
pub struct LibraryBrowser {
    state: ComponentState,
    fs_navigator: FSNavigator,
}

impl Component for LibraryBrowser {
    fn new() -> Self {
        // Start in the current directory
        let fs_navigator = FSNavigator::new(PathBuf::from("."));
        let mut browser = Self {
            state: ComponentState::default(),
            fs_navigator,
        };
        
        // Initial directory scan
        if let Err(e) = browser.fs_navigator.scan_current_dir() {
            eprintln!("Error scanning directory: {}", e);
        }
        
        browser
    }

    fn render(&self, frame: &mut Frame, area: Rect, focused: bool, theme: &Theme) {
        let block = create_block("Library Browser", focused, theme);
        
        // Create a list of entries
        let fs_state = self.fs_navigator.state();
        let entries: Vec<ListItem> = fs_state
            .entries()
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                let prefix = if entry.is_dir() { "📁 " } else { "📄 " };
                let style = if Some(i) == fs_state.selected_index() {
                    theme.get_style("list_selected")
                } else {
                    theme.get_style("list_item")
                };
                
                ListItem::new(format!("{}{}", prefix, entry.name()))
                    .style(style)
            })
            .collect();

        let list = List::new(entries)
            .block(block)
            .highlight_style(theme.get_style("list_selected"));

        frame.render_widget(list, area);
    }

    fn update(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::NavigateUp => {
                if let Err(e) = self.fs_navigator.handle_action(FSAction::Select(
                    self.fs_navigator.state().selected_index()
                        .map(|i| if i > 0 { i - 1 } else { 0 })
                        .unwrap_or(0)
                )) {
                    eprintln!("Error navigating: {}", e);
                }
            }
            Action::NavigateDown => {
                let max_index = self.fs_navigator.state().entries().len().saturating_sub(1);
                if let Err(e) = self.fs_navigator.handle_action(FSAction::Select(
                    self.fs_navigator.state().selected_index()
                        .map(|i| if i < max_index { i + 1 } else { max_index })
                        .unwrap_or(0)
                )) {
                    eprintln!("Error navigating: {}", e);
                }
            }
            Action::Select => {
                if let Err(e) = self.fs_navigator.handle_action(FSAction::NavigateToSelected) {
                    eprintln!("Error selecting entry: {}", e);
                }
            }
            Action::Back => {
                if let Err(e) = self.fs_navigator.handle_action(FSAction::NavigateToParent) {
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
            Event::Key(key_event) => match key_event {
                KeyEvent::Up => Some(Action::NavigateUp),
                KeyEvent::Down => Some(Action::NavigateDown),
                KeyEvent::Enter => Some(Action::Select),
                KeyEvent::Esc => Some(Action::Back),
                _ => None,
            },
            _ => None,
        }
    }
}
