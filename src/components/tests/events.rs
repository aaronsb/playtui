use crate::components::Component;
use super::mock::MockComponent;
use crate::events::{Event, Action, KeyEvent, SystemEvent};

#[test]
fn test_navigation_events_require_focus() {
    let mut component = MockComponent::new();
    
    // When not focused
    component.set_focused(false);
    assert_eq!(component.handle_event(Event::Key(KeyEvent::Left)), Some(Action::Refresh));
    assert_eq!(component.handle_event(Event::Key(KeyEvent::Right)), Some(Action::Refresh));
    assert_eq!(component.handle_event(Event::Key(KeyEvent::Up)), Some(Action::Refresh));
    assert_eq!(component.handle_event(Event::Key(KeyEvent::Down)), Some(Action::Refresh));
    assert_eq!(component.handle_event(Event::Key(KeyEvent::Enter)), Some(Action::Refresh));
    assert_eq!(component.handle_event(Event::Key(KeyEvent::Tab)), Some(Action::Refresh));
}

#[test]
fn test_playback_events_ignore_focus() {
    let mut component = MockComponent::new();
    
    // When not focused, playback events should still be handled
    component.set_focused(false);
    assert_eq!(component.handle_event(Event::Key(KeyEvent::Space)), Some(Action::Refresh));
    assert_eq!(component.handle_event(Event::Key(KeyEvent::Play)), Some(Action::Refresh));
    assert_eq!(component.handle_event(Event::Key(KeyEvent::Pause)), Some(Action::Refresh));
    assert_eq!(component.handle_event(Event::Key(KeyEvent::Stop)), Some(Action::Refresh));
    assert_eq!(component.handle_event(Event::Key(KeyEvent::Next)), Some(Action::Refresh));
    assert_eq!(component.handle_event(Event::Key(KeyEvent::Previous)), Some(Action::Refresh));
}

#[test]
fn test_system_events_ignore_focus() {
    let mut component = MockComponent::new();
    
    // System events should be handled regardless of focus
    component.set_focused(false);
    assert_eq!(
        component.handle_event(Event::System(SystemEvent::TrackLoaded)),
        Some(Action::Refresh)
    );
    assert_eq!(
        component.handle_event(Event::System(SystemEvent::TrackEnded)),
        Some(Action::Refresh)
    );
}

#[test]
fn test_focused_event_handling() {
    let mut component = MockComponent::new();
    
    // Test when focused
    component.set_focused(true);
    
    // Navigation events
    assert_eq!(component.handle_event(Event::Key(KeyEvent::Left)), Some(Action::Refresh));
    assert_eq!(component.handle_event(Event::Key(KeyEvent::Right)), Some(Action::Refresh));
    
    // Playback events
    assert_eq!(component.handle_event(Event::Key(KeyEvent::Play)), Some(Action::Refresh));
    assert_eq!(component.handle_event(Event::Key(KeyEvent::Stop)), Some(Action::Refresh));
    
    // System events
    assert_eq!(
        component.handle_event(Event::System(SystemEvent::TrackLoaded)),
        Some(Action::Refresh)
    );
}
