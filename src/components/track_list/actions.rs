use crate::events::{Event, Action, KeyEvent, MouseEvent, PlaylistAction};
use super::state::TrackListState;

pub fn handle_event(state: &mut TrackListState, event: Event) -> Option<Action> {
    // Only handle events if component is focused (except for specific system events)
    if !state.focused() {
        return match event {
            Event::System(_) => Some(Action::Refresh), // Always handle system events
            _ => None,
        };
    }

    match event {
        Event::Key(key_event) => handle_key_event(state, key_event),
        Event::Mouse(mouse_event) => handle_mouse_event(state, mouse_event),
        _ => None,
    }
}

pub fn handle_action(state: &mut TrackListState, action: Action) -> Option<Action> {
    match action {
        Action::Select => {
            if let Some(index) = state.selected_index {
                if index < state.tracks.len() {
                    Some(Action::Playlist(PlaylistAction::SelectTrack(index)))
                } else {
                    None
                }
            } else {
                None
            }
        }
        Action::Back => {
            state.clear_selection();
            Some(Action::Refresh)
        }
        _ => None,
    }
}

fn handle_key_event(state: &mut TrackListState, key_event: KeyEvent) -> Option<Action> {
    match key_event {
        KeyEvent::Up => {
            if state.select_previous() {
                Some(Action::Refresh)
            } else {
                None
            }
        },
        KeyEvent::Down => {
            if state.select_next() {
                Some(Action::Refresh)
            } else {
                None
            }
        },
        KeyEvent::Enter => Some(Action::Select),
        KeyEvent::Escape => Some(Action::Back),
        _ => None,
    }
}

fn handle_mouse_event(state: &mut TrackListState, mouse_event: MouseEvent) -> Option<Action> {
    if state.is_empty() {
        return None;
    }

    match mouse_event {
        MouseEvent::Click { x: _, y } => {
            // Convert y coordinate to list index, accounting for the border
            let clicked_index = (y as usize).saturating_sub(1); // -1 for the border
            
            // Check if click is within valid range
            if clicked_index < state.len() {
                // If clicking the same item that's already selected, treat as Enter key
                if state.selected_index == Some(clicked_index) {
                    Some(Action::Select)
                } else {
                    state.select_index(clicked_index);
                    Some(Action::Refresh)
                }
            } else {
                None
            }
        },
        MouseEvent::Scroll { delta } => {
            if delta < 0 {
                // Scroll down
                if state.select_next() {
                    Some(Action::Refresh)
                } else {
                    None
                }
            } else {
                // Scroll up
                if state.select_previous() {
                    Some(Action::Refresh)
                } else {
                    None
                }
            }
        }
    }
}
