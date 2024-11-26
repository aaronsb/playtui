use super::*;
use crate::events::{KeyEvent, MouseEvent, PlaylistAction};

fn setup_track_list() -> TrackList {
    let mut track_list = TrackList::new();
    track_list.state.tracks = vec![
        "Track 1".to_string(),
        "Track 2".to_string(),
        "Track 3".to_string(),
    ];
    track_list
}

#[test]
fn test_track_list_navigation() {
    let mut track_list = setup_track_list();
    track_list.set_focused(true);
    
    // Initial state
    assert_eq!(track_list.state.selected_index, None);

    // Navigate down
    track_list.handle_event(Event::Key(KeyEvent::Down));
    assert_eq!(track_list.state.selected_index, Some(0));

    // Navigate down again
    track_list.handle_event(Event::Key(KeyEvent::Down));
    assert_eq!(track_list.state.selected_index, Some(1));

    // Navigate up
    track_list.handle_event(Event::Key(KeyEvent::Up));
    assert_eq!(track_list.state.selected_index, Some(0));

    // Navigate up at top (should stay at 0)
    track_list.handle_event(Event::Key(KeyEvent::Up));
    assert_eq!(track_list.state.selected_index, Some(0));
}

#[test]
fn test_track_list_bounds() {
    let mut track_list = setup_track_list();
    track_list.set_focused(true);
    
    // Navigate past end
    for _ in 0..5 {
        track_list.handle_event(Event::Key(KeyEvent::Down));
    }
    assert_eq!(track_list.state.selected_index, Some(2)); // Should stop at last index

    // Navigate back up
    track_list.handle_event(Event::Key(KeyEvent::Up));
    assert_eq!(track_list.state.selected_index, Some(1));
}

#[test]
fn test_track_list_selection() {
    let mut track_list = setup_track_list();
    track_list.set_focused(true);
    
    // Select without selection
    assert_eq!(track_list.update(Action::Select), None);

    // Navigate and select
    track_list.handle_event(Event::Key(KeyEvent::Down));
    assert_eq!(
        track_list.update(Action::Select),
        Some(Action::Playlist(PlaylistAction::SelectTrack(0)))
    );

    // Clear selection
    track_list.update(Action::Back);
    assert_eq!(track_list.state.selected_index, None);
}

#[test]
fn test_unfocused_events() {
    let mut track_list = setup_track_list();
    track_list.set_focused(false);
    
    // Test that unfocused component ignores navigation
    assert_eq!(track_list.handle_event(Event::Key(KeyEvent::Up)), None);
    assert_eq!(track_list.handle_event(Event::Key(KeyEvent::Down)), None);
    assert_eq!(track_list.handle_event(Event::Key(KeyEvent::Enter)), None);
    assert_eq!(track_list.state.selected_index, None);
}

#[test]
fn test_empty_list_navigation() {
    let mut track_list = TrackList::new();
    track_list.set_focused(true);
    
    // Test that navigation on empty list returns None
    assert_eq!(track_list.handle_event(Event::Key(KeyEvent::Up)), None);
    assert_eq!(track_list.handle_event(Event::Key(KeyEvent::Down)), None);
    assert_eq!(track_list.state.selected_index, None);
}

#[test]
fn test_mouse_click() {
    let mut track_list = setup_track_list();
    track_list.set_focused(true);
    
    // Click first item (y=1 due to border)
    let result = track_list.handle_event(Event::Mouse(MouseEvent::Click { x: 0, y: 1 }));
    assert_eq!(track_list.state.selected_index, Some(0));
    assert_eq!(result, Some(Action::Refresh));

    // Click same item again - should trigger selection
    let result = track_list.handle_event(Event::Mouse(MouseEvent::Click { x: 0, y: 1 }));
    assert_eq!(result, Some(Action::Select));

    // Click out of bounds
    let result = track_list.handle_event(Event::Mouse(MouseEvent::Click { x: 0, y: 10 }));
    assert_eq!(result, None);
}

#[test]
fn test_mouse_scroll() {
    let mut track_list = setup_track_list();
    track_list.set_focused(true);
    
    // Initial selection
    track_list.state.selected_index = Some(1);

    // Scroll up
    let result = track_list.handle_event(Event::Mouse(MouseEvent::Scroll { delta: 1 }));
    assert_eq!(track_list.state.selected_index, Some(0));
    assert_eq!(result, Some(Action::Refresh));

    // Scroll down
    let result = track_list.handle_event(Event::Mouse(MouseEvent::Scroll { delta: -1 }));
    assert_eq!(track_list.state.selected_index, Some(1));
    assert_eq!(result, Some(Action::Refresh));
}
