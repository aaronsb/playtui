use super::*;
use crate::events::{Event, KeyEvent, Action};
use ratatui::{
    backend::TestBackend,
    Terminal,
    layout::Rect,
};

#[test]
fn test_volume_control_new() {
    let control = VolumeControl::new();
    assert!(!control.focused(), "Should not be focused by default");
    assert_eq!(control.volume, 50, "Default volume should be 50%");
}

#[test]
fn test_volume_control_focus() {
    let mut control = VolumeControl::new();
    assert!(!control.focused(), "Should start unfocused");
    
    control.set_focused(true);
    assert!(control.focused(), "Should be focused after set_focused(true)");
    
    control.set_focused(false);
    assert!(!control.focused(), "Should be unfocused after set_focused(false)");
}

#[test]
fn test_volume_control_bounds() {
    let mut control = VolumeControl::new();
    
    // Test upper bound
    for _ in 0..30 {  // More than enough to reach 100%
        control.update(Action::VolumeUp);
    }
    assert_eq!(control.volume, 100, "Volume should not exceed 100%");
    
    // Test lower bound
    for _ in 0..30 {  // More than enough to reach 0%
        control.update(Action::VolumeDown);
    }
    assert_eq!(control.volume, 0, "Volume should not go below 0%");
}

#[test]
fn test_volume_control_increment() {
    let mut control = VolumeControl::new();
    let initial = control.volume;
    
    // Test volume up
    let result = control.update(Action::VolumeUp);
    assert_eq!(control.volume, initial + 5, "Volume should increase by 5");
    assert!(matches!(result, Some(Action::SetVolume(vol)) if vol == initial + 5), 
        "Should return SetVolume action with new volume");
    
    // Test volume down
    let before_down = control.volume;
    let result = control.update(Action::VolumeDown);
    assert_eq!(control.volume, before_down - 5, "Volume should decrease by 5");
    assert!(matches!(result, Some(Action::SetVolume(vol)) if vol == before_down - 5),
        "Should return SetVolume action with new volume");
}

#[test]
fn test_volume_control_set_volume() {
    let mut control = VolumeControl::new();
    
    // Test setting specific volume
    let result = control.update(Action::SetVolume(75));
    assert_eq!(control.volume, 75, "Volume should be set to 75");
    assert!(result.is_none(), "SetVolume should not produce a new action");
}

#[test]
fn test_volume_control_event_handling_focused() {
    let mut control = VolumeControl::new();
    control.set_focused(true);
    
    // Test Up key
    let result = control.handle_event(Event::Key(KeyEvent::Up));
    assert!(matches!(result, Some(Action::VolumeUp)), 
        "Up key should produce VolumeUp action when focused");
    
    // Test Down key
    let result = control.handle_event(Event::Key(KeyEvent::Down));
    assert!(matches!(result, Some(Action::VolumeDown)), 
        "Down key should produce VolumeDown action when focused");
    
    // Test other key
    let result = control.handle_event(Event::Key(KeyEvent::Enter));
    assert!(result.is_none(), "Other keys should not produce actions");
}

#[test]
fn test_volume_control_event_handling_unfocused() {
    let mut control = VolumeControl::new();
    control.set_focused(false);
    
    // Test Up key when unfocused
    let result = control.handle_event(Event::Key(KeyEvent::Up));
    assert!(result.is_none(), "Should not handle events when unfocused");
    
    // Test Down key when unfocused
    let result = control.handle_event(Event::Key(KeyEvent::Down));
    assert!(result.is_none(), "Should not handle events when unfocused");
}

#[test]
fn test_volume_control_render() {
    let control = VolumeControl::new();
    let theme = crate::theme::Theme::load_default().unwrap();
    let area = Rect::new(0, 0, 20, 3);
    let backend = TestBackend::new(20, 3);
    let mut terminal = Terminal::new(backend).unwrap();
    
    // Test unfocused render
    terminal.draw(|frame| {
        control.render(frame, area, false, &theme);
    }).unwrap();
    
    let buffer = terminal.backend().buffer();
    let content = buffer.content.iter()
        .map(|cell| cell.symbol.clone())
        .collect::<String>();
    assert!(content.contains("Volume: 50%"), "Should display current volume");
    
    // Test focused render
    terminal.draw(|frame| {
        control.render(frame, area, true, &theme);
    }).unwrap();
    
    let buffer = terminal.backend().buffer();
    let content = buffer.content.iter()
        .map(|cell| cell.symbol.clone())
        .collect::<String>();
    assert!(content.contains("Volume: 50%"), "Should display current volume when focused");
}
