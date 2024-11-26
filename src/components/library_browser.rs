use ratatui::{
    prelude::*,
    widgets::{List, ListItem, ListState},
};
use super::{Component, ComponentState, create_block};
use crate::events::{Event, Action, KeyEvent, NavigationEvent, EventHandler, EventResult};
use crate::components::filesystem::{FSNavigator, FSAction};
use crate::theme::Theme;
use std::{path::PathBuf, cell::RefCell};

#[derive(Clone)]
pub struct LibraryBrowser {
    state: ComponentState,
    fs_navigator: RefCell<FSNavigator>,
}

impl LibraryBrowser {
    // Helper method to process events
    fn process_event(&mut self, event: &Event) -> Option<Action> {
        if !self.focused() {
            return None;
        }

        match event {
            Event::Key(key_event) => match key_event {
                KeyEvent::Up => {
                    let mut navigator = self.fs_navigator.borrow_mut();
                    let current_index = navigator.state().selected_index().unwrap_or(0);
                    let new_index = if current_index > 0 { current_index - 1 } else { 0 };
                    
                    if let Err(e) = navigator.handle_action(FSAction::Select(new_index)) {
                        eprintln!("Error navigating: {}", e);
                    }
                    Some(Action::Refresh)
                },
                KeyEvent::Down => {
                    let mut navigator = self.fs_navigator.borrow_mut();
                    let current_index = navigator.state().selected_index().unwrap_or(0);
                    let max_index = navigator.state().entries().len().saturating_sub(1);
                    let new_index = if current_index < max_index { current_index + 1 } else { max_index };
                    
                    if let Err(e) = navigator.handle_action(FSAction::Select(new_index)) {
                        eprintln!("Error navigating: {}", e);
                    }
                    Some(Action::Refresh)
                },
                KeyEvent::Right | KeyEvent::Enter => {
                    if let Err(e) = self.fs_navigator.borrow_mut().handle_action(FSAction::NavigateToSelected) {
                        eprintln!("Error selecting entry: {}", e);
                    }
                    Some(Action::Refresh)
                },
                KeyEvent::Left | KeyEvent::Escape => {
                    if let Err(e) = self.fs_navigator.borrow_mut().handle_action(FSAction::NavigateToParent) {
                        eprintln!("Error navigating to parent: {}", e);
                    }
                    Some(Action::Refresh)
                },
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

impl EventHandler for LibraryBrowser {
    fn can_handle(&self, event: &Event) -> bool {
        match event {
            Event::Key(KeyEvent::Tab) |
            Event::Key(KeyEvent::Quit) => true,
            
            Event::Key(KeyEvent::Enter) |
            Event::Key(KeyEvent::Left) |
            Event::Key(KeyEvent::Right) |
            Event::Key(KeyEvent::Up) |
            Event::Key(KeyEvent::Down) |
            Event::Navigation(_) => self.focused(),
            
            _ => false
        }
    }

    fn handle_event(&mut self, event: &Event) -> EventResult<Option<Action>> {
        if !self.can_handle(event) {
            return Ok(None);
        }
        Ok(self.process_event(event))
    }
}

impl Component for LibraryBrowser {
    fn new() -> Self {
        // Start in the current directory
        let fs_navigator = FSNavigator::new(PathBuf::from("."));
        let browser = Self {
            state: ComponentState::default(),
            fs_navigator: RefCell::new(fs_navigator),
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
        
        // Create list items with proper styling
        let items: Vec<ListItem> = entries
            .iter()
            .enumerate()
            .map(|(index, entry)| {
                let prefix = if entry.is_dir() { "📁 " } else { "📄 " };
                let style = if Some(index) == selected {
                    if focused {
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::Yellow)
                            .add_modifier(Modifier::BOLD)
                    } else {
                        Style::default()
                            .fg(Color::Black)
                            .bg(Color::DarkGray)
                    }
                } else {
                    if focused {
                        theme.get_style("list_item")
                    } else {
                        Style::default().fg(Color::DarkGray)
                    }
                };
                
                ListItem::new(format!("{}{}", prefix, entry.name()))
                    .style(style)
            })
            .collect();

        // Create list widget with explicit highlight style
        let list = List::new(items)
            .block(block)
            .highlight_style(
                if focused {
                    Style::default()
                        .bg(Color::Yellow)
                        .fg(Color::Black)
                        .add_modifier(Modifier::BOLD)
                } else {
                    Style::default()
                        .bg(Color::DarkGray)
                        .fg(Color::Black)
                }
            );

        // Create and render list state
        let mut list_state = ListState::default();
        list_state.select(selected);
        frame.render_stateful_widget(list, area, &mut list_state);
    }

    fn update(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::NavigateUp | Action::NavigateDown | 
            Action::NavigateRight | Action::NavigateLeft |
            Action::Select | Action::Back => {
                // Let process_event handle these actions directly
                self.process_event(&Event::Key(match action {
                    Action::NavigateUp => KeyEvent::Up,
                    Action::NavigateDown => KeyEvent::Down,
                    Action::NavigateRight | Action::Select => KeyEvent::Enter,
                    Action::NavigateLeft | Action::Back => KeyEvent::Left,
                    _ => unreachable!(),
                }))
            }
            _ => None,
        }
    }

    fn focused(&self) -> bool {
        self.state.focused
    }

    fn set_focused(&mut self, focused: bool) {
        self.state.focused = focused;
    }

    fn handle_event(&mut self, event: Event) -> Option<Action> {
        self.process_event(&event)
    }
}
