use super::*;
use crate::events::{Event, Action, KeyEvent, MouseEvent};
use ratatui::prelude::*;

#[test]
fn test_default_volume() {
    let control = VolumeControl::new();
    assert_eq!(control.state.get_volume(), 50, "Default volume should be 50%");
}

#[test]
fn test_key_events() {
    let mut control = VolumeControl::new();
    control.set_focused(true);
    
    // Test volume up
    let result = control.handle_event(Event::Key(KeyEvent::Up));
    assert_eq!(result, Some(Action::VolumeUp));
    control.update(Action::VolumeUp);
    assert_eq!(control.state.get_volume(), 55, "Volume should increase by 5");
    
    // Test volume down
    let result = control.handle_event(Event::Key(KeyEvent::Down));
    assert_eq!(result, Some(Action::VolumeDown));
    control.update(Action::VolumeDown);
    assert_eq!(control.state.get_volume(), 50, "Volume should decrease by 5");
}

#[test]
fn test_volume_limits() {
    let mut control = VolumeControl::new();
    
    // Test upper limit
    for _ in 0..20 {
        control.update(Action::VolumeUp);
    }
    assert_eq!(control.state.get_volume(), 100, "Volume should not exceed 100%");
    
    // Test lower limit
    for _ in 0..30 {
        control.update(Action::VolumeDown);
    }
    assert_eq!(control.state.get_volume(), 0, "Volume should not go below 0%");
}

#[test]
fn test_volume_up() {
    let mut control = VolumeControl::new();
    let initial = control.state.get_volume();
    
    control.update(Action::VolumeUp);
    
    assert_eq!(control.state.get_volume(), initial + 5, "Volume should increase by 5");
}

#[test]
fn test_volume_down() {
    let mut control = VolumeControl::new();
    let before_down = control.state.get_volume();
    control.update(Action::VolumeDown);
    assert_eq!(control.state.get_volume(), before_down - 5, "Volume should decrease by 5");
}

#[test]
fn test_set_volume() {
    let mut control = VolumeControl::new();
    
    // Test setting volume directly
    control.update(Action::SetVolume(75));
    assert_eq!(control.state.get_volume(), 75, "Volume should be set to 75");
    
    // Test setting volume above max
    control.update(Action::SetVolume(150));
    assert_eq!(control.state.get_volume(), 100, "Volume should be capped at 100");
}

#[test]
fn test_unfocused_control() {
    let mut control = VolumeControl::new();
    control.set_focused(false);
    
    let result = control.handle_event(Event::Key(KeyEvent::Up));
    assert_eq!(result, None, "Unfocused control should not handle events");
}
