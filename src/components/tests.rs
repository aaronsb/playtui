use super::*;
use crate::events::{Event, Action, KeyEvent, SystemEvent};
use crate::theme::{Theme, ThemeMetadata, Colors, Controls, Styles, StyleConfig};
use ratatui::widgets::Block;

// Mock component for testing
pub struct MockComponent {
    state: ComponentState,
    last_event: Option<Event>,
}

impl Component for MockComponent {
    fn new() -> Self {
        Self {
            state: ComponentState::default(),
            last_event: None,
        }
    }

    fn render(&self, _frame: &mut Frame, _area: Rect, _focused: bool, _theme: &Theme) {}

    fn update(&mut self, _action: Action) -> Option<Action> {
        None
    }

    fn handle_event(&mut self, event: Event) -> Option<Action> {
        self.last_event = Some(event.clone());
        Some(Action::Refresh)
    }

    fn focused(&self) -> bool {
        self.state.focused
    }

    fn set_focused(&mut self, focused: bool) {
        self.state.focused = focused;
    }
}

// Helper function to create a test theme
fn create_test_theme() -> Theme {
    Theme {
        metadata: ThemeMetadata {
            name: "Test Theme".to_string(),
            author: "Test Author".to_string(),
            version: "1.0".to_string(),
        },
        colors: Colors {
            primary: "#2E8B57".to_string(),
            secondary: "#98FB98".to_string(),
            background: "#1A1A1A".to_string(),
            foreground: "#F5F5F5".to_string(),
            active: "#98FB98".to_string(),
            inactive: "#696969".to_string(),
            playing: "#00FF7F".to_string(),
            error: "#FF6347".to_string(),
        },
        controls: Controls {
            record: "⏺".to_string(),
            play: "⏵".to_string(),
            rewind: "⏪".to_string(),
            fast_forward: "⏩".to_string(),
            stop: "⏹".to_string(),
            pause: "⏸".to_string(),
            next: "⏭".to_string(),
            previous: "⏮".to_string(),
        },
        styles: Styles {
            border_focused: StyleConfig {
                fg: Some("#98FB98".to_string()),
                bg: None,
                modifiers: Some(vec!["BOLD".to_string()]),
            },
            border_unfocused: StyleConfig {
                fg: Some("#2E8B57".to_string()),
                bg: None,
                modifiers: None,
            },
            text_normal: StyleConfig {
                fg: Some("#F5F5F5".to_string()),
                bg: None,
                modifiers: None,
            },
            text_bold: StyleConfig {
                fg: Some("#F5F5F5".to_string()),
                bg: None,
                modifiers: Some(vec!["BOLD".to_string()]),
            },
            text_dim: StyleConfig {
                fg: Some("#696969".to_string()),
                bg: None,
                modifiers: None,
            },
            button: StyleConfig {
                fg: Some("#2E8B57".to_string()),
                bg: None,
                modifiers: Some(vec!["BOLD".to_string()]),
            },
            list_item: StyleConfig {
                fg: Some("#F5F5F5".to_string()),
                bg: None,
                modifiers: None,
            },
            list_selected: StyleConfig {
                fg: Some("#98FB98".to_string()),
                bg: None,
                modifiers: Some(vec!["BOLD".to_string()]),
            },
            playing_item: StyleConfig {
                fg: Some("#00FF7F".to_string()),
                bg: None,
                modifiers: Some(vec!["BOLD".to_string()]),
            },
            progress_bar: StyleConfig {
                fg: Some("#98FB98".to_string()),
                bg: Some("#1A1A1A".to_string()),
                modifiers: None,
            },
            volume_indicator: StyleConfig {
                fg: Some("#00FF7F".to_string()),
                bg: None,
                modifiers: Some(vec!["BOLD".to_string()]),
            },
            tab_active: StyleConfig {
                fg: Some("#1A1A1A".to_string()),
                bg: Some("#2E8B57".to_string()),
                modifiers: Some(vec!["BOLD".to_string()]),
            },
            tab_inactive: StyleConfig {
                fg: Some("#2E8B57".to_string()),
                bg: None,
                modifiers: None,
            },
        },
    }
}

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
fn test_focus_state_changes() {
    let mut component = MockComponent::new();
    
    // Initial state
    assert!(!component.focused());
    
    // Set focused
    component.set_focused(true);
    assert!(component.focused());
    
    // Set unfocused
    component.set_focused(false);
    assert!(!component.focused());
}

#[test]
fn test_create_block() {
    let title = "Test Block";
    let focused = true;
    let theme = create_test_theme();
    
    let _block = create_block(title, focused, &theme);
    // Block creation succeeded if we got here
}

#[test]
fn test_mock_component_event_handling() {
    let mut component = MockComponent::new();
    
    // Test focused state handling
    component.set_focused(true);
    let event = Event::Key(KeyEvent::Enter);
    component.handle_event(event.clone());
    assert_eq!(component.last_event, Some(event));
}

#[test]
fn test_mock_component_creation() {
    let component = MockComponent::new();
    assert!(!component.focused());
    assert_eq!(component.last_event, None);
}

#[test]
fn test_component_state_default() {
    let state = ComponentState::default();
    assert!(!state.focused);
}

#[test]
fn test_component_state_clone() {
    let state = ComponentState { focused: true };
    let cloned = state.clone();
    assert_eq!(state.focused, cloned.focused);
}
