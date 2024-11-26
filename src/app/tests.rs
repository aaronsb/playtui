use super::*;
use crate::events::{Event, KeyEvent};

#[test]
fn test_global_navigation_events() {
    let mut app = App::new().unwrap();
    let initial_focus = app.focus_manager.current_focus().to_string();
    
    // Tab should always change focus
    app.handle_event(Event::Key(KeyEvent::Tab)).unwrap();
    assert_ne!(app.focus_manager.current_focus(), initial_focus);
}

#[test]
fn test_frame_specific_events() {
    let mut app = App::new().unwrap();
    let focused_component = app.focus_manager.current_focus().to_string();
    
    // Arrow keys should only work on focused component
    app.handle_event(Event::Key(KeyEvent::Left)).unwrap();
    assert_eq!(app.focus_manager.current_focus(), focused_component);
}

#[test]
fn test_global_hotkeys() {
    let mut app = App::new().unwrap();
    let initial_focus = app.focus_manager.current_focus().to_string();
    
    // Space should work regardless of focus
    app.handle_event(Event::Key(KeyEvent::Space)).unwrap();
    assert_eq!(app.focus_manager.current_focus(), initial_focus);
}
