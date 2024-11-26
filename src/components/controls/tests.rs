use super::*;
use crate::events::{Event, Action, KeyEvent, NavigationEvent, MouseEvent};

#[test]
fn test_navigation_events_work_regardless_of_focus() {
    let mut controls = Controls::new();
    
    // Test when not focused
    controls.set_focused(false);
    assert!(!controls.focused());
    
    // Right navigation should work and move to next button
    let result = controls.handle_event(Event::Navigation(NavigationEvent::Right));
    assert_eq!(controls.focused_button, 1);
    assert_eq!(result, Some(Action::Refresh));
    
    // Left navigation should work and move to previous button
    let result = controls.handle_event(Event::Navigation(NavigationEvent::Left));
    assert_eq!(controls.focused_button, 0);
    assert_eq!(result, Some(Action::Refresh));
    
    // Test when focused
    controls.set_focused(true);
    assert!(controls.focused());
    
    // Right navigation should still work
    let result = controls.handle_event(Event::Navigation(NavigationEvent::Right));
    assert_eq!(controls.focused_button, 1);
    assert_eq!(result, Some(Action::Refresh));
}

#[test]
fn test_key_events_blocked_when_not_focused() {
    let mut controls = Controls::new();
    controls.set_focused(false);
    assert!(!controls.focused());
    
    // Key events should return None when not focused
    let result = controls.handle_event(Event::Key(KeyEvent::Enter));
    assert_eq!(result, None);
    
    // But navigation key events should be converted to navigation actions
    let result = controls.handle_event(Event::Key(KeyEvent::Right));
    assert_eq!(result, Some(Action::NavigateRight));
}

#[test]
fn test_mouse_events_blocked_when_not_focused() {
    let mut controls = Controls::new();
    controls.set_focused(false);
    assert!(!controls.focused());
    
    let result = controls.handle_event(Event::Mouse(MouseEvent::Click { x: 0, y: 0 }));
    assert_eq!(result, None);
}

#[test]
fn test_button_focus_wraps_correctly() {
    let mut controls = Controls::new();
    controls.focused_section = Section::Controls;
    
    // Test wrapping from 0 to 7 when going left
    controls.focused_button = 0;
    let result = controls.handle_event(Event::Navigation(NavigationEvent::Left));
    assert_eq!(controls.focused_button, 7);
    assert_eq!(result, Some(Action::Refresh));
    
    // Test wrapping from 7 to 0 when going right
    controls.focused_button = 7;
    let result = controls.handle_event(Event::Navigation(NavigationEvent::Right));
    assert_eq!(controls.focused_button, 0);
    assert_eq!(result, Some(Action::Refresh));
}

#[test]
fn test_volume_section_navigation() {
    let mut controls = Controls::new();
    controls.focused_section = Section::Controls;
    
    // Test moving from Controls to Volume section
    let result = controls.handle_event(Event::Navigation(NavigationEvent::Down));
    assert_eq!(controls.focused_section, Section::Volume);
    assert_eq!(result, Some(Action::Refresh));
    
    // Test moving from Volume back to Controls section
    let result = controls.handle_event(Event::Navigation(NavigationEvent::Up));
    assert_eq!(controls.focused_section, Section::Controls);
    assert_eq!(result, Some(Action::Refresh));
}
