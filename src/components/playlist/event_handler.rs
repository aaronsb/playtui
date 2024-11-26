use crate::events::{Event, Action, MouseEvent, KeyEvent, PlaylistAction};
use super::{Playlist, selection};

pub(super) fn handle_event_internal(playlist: &mut Playlist, event: Event) -> Option<Action> {
    match event {
        Event::Key(key_event) => handle_key_event(playlist, key_event),
        Event::Mouse(mouse_event) => handle_mouse_event(playlist, mouse_event),
        _ => None,
    }
}

fn handle_key_event(playlist: &mut Playlist, event: KeyEvent) -> Option<Action> {
    match event {
        KeyEvent::Next => selection::select_next(playlist),
        KeyEvent::Previous => selection::select_previous(playlist),
        _ => None,
    }
}

fn handle_mouse_event(playlist: &mut Playlist, event: MouseEvent) -> Option<Action> {
    match event {
        MouseEvent::Click { x: _, y } => {
            // Calculate which track was clicked based on y coordinate
            // Accounting for border and title
            let index = y.saturating_sub(1) as usize + playlist.scroll_offset;
            if index < playlist.tracks.len() {
                Some(Action::Playlist(PlaylistAction::SelectTrack(index)))
            } else {
                None
            }
        }
        MouseEvent::Scroll { delta } => {
            // Update scroll offset
            if delta < 0 && playlist.scroll_offset < playlist.tracks.len().saturating_sub(1) {
                playlist.scroll_offset += 1;
            } else if delta > 0 && playlist.scroll_offset > 0 {
                playlist.scroll_offset -= 1;
            }
            None
        }
    }
}
