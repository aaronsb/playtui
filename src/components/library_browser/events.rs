use crate::events::{Event, Action, KeyEvent, NavigationEvent, EventHandler, EventResult, MouseEvent};
use super::state::LibraryBrowserState;

pub fn process_event(state: &mut LibraryBrowserState, event: &Event) -> Option<Action> {
    if !state.focused() {
        return None;
    }

    match event {
        Event::Key(key_event) => handle_key_event(state, key_event),
        Event::Mouse(mouse_event) => handle_mouse_event(state, mouse_event),
        Event::Navigation(nav_event) => handle_navigation_event(nav_event),
        _ => None,
    }
}

fn handle_key_event(state: &mut LibraryBrowserState, key_event: &KeyEvent) -> Option<Action> {
    match key_event {
        KeyEvent::Up => {
            if let Err(e) = state.navigate_up() {
                eprintln!("Error navigating: {}", e);
            }
            Some(Action::Refresh)
        },
        KeyEvent::Down => {
            if let Err(e) = state.navigate_down() {
                eprintln!("Error navigating: {}", e);
            }
            Some(Action::Refresh)
        },
        KeyEvent::Right | KeyEvent::Enter => {
            if let Err(e) = state.navigate_to_selected() {
                eprintln!("Error selecting entry: {}", e);
            }
            Some(Action::Refresh)
        },
        KeyEvent::Left | KeyEvent::Escape => {
            if let Err(e) = state.navigate_to_parent() {
                eprintln!("Error navigating to parent: {}", e);
            }
            Some(Action::Refresh)
        },
        _ => None,
    }
}

fn handle_mouse_event(state: &mut LibraryBrowserState, mouse_event: &MouseEvent) -> Option<Action> {
    match mouse_event {
        MouseEvent::Click { x: _, y } => {
            // Convert y coordinate to list index, accounting for the border
            let clicked_index = *y as usize - 1; // -1 for the border
            let entries_len = state.get_entries().len();
            let max_index = entries_len.saturating_sub(1);
            
            // Check if click is within valid range
            if clicked_index <= max_index {
                // First select the clicked item
                if let Err(e) = state.select_index(clicked_index) {
                    eprintln!("Error selecting item: {}", e);
                    return Some(Action::Refresh);
                }
                
                // If clicking the same item that's already selected, treat as Enter key
                if state.get_selected_index() == Some(clicked_index) {
                    if let Err(e) = state.navigate_to_selected() {
                        eprintln!("Error navigating to selected: {}", e);
                    }
                }
            }
            Some(Action::Refresh)
        },
        MouseEvent::Scroll { delta } => {
            if *delta < 0 {
                if let Err(e) = state.navigate_down() {
                    eprintln!("Error navigating: {}", e);
                }
            } else {
                if let Err(e) = state.navigate_up() {
                    eprintln!("Error navigating: {}", e);
                }
            }
            Some(Action::Refresh)
        }
    }
}

fn handle_navigation_event(nav_event: &NavigationEvent) -> Option<Action> {
    match nav_event {
        NavigationEvent::Up => Some(Action::NavigateUp),
        NavigationEvent::Down => Some(Action::NavigateDown),
        NavigationEvent::Right => Some(Action::NavigateRight),
        NavigationEvent::Left => Some(Action::NavigateLeft),
    }
}

pub fn can_handle_event(state: &LibraryBrowserState, event: &Event) -> bool {
    match event {
        Event::Key(KeyEvent::Tab) |
        Event::Key(KeyEvent::Quit) => true,
        
        Event::Key(KeyEvent::Enter) |
        Event::Key(KeyEvent::Left) |
        Event::Key(KeyEvent::Right) |
        Event::Key(KeyEvent::Up) |
        Event::Key(KeyEvent::Down) |
        Event::Mouse(_) |
        Event::Navigation(_) => state.focused(),
        
        _ => false
    }
}

pub fn handle_event(state: &mut LibraryBrowserState, event: &Event) -> EventResult<Option<Action>> {
    if !can_handle_event(state, event) {
        return Ok(None);
    }
    Ok(process_event(state, event))
}
