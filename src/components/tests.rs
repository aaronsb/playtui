use super::*;
use crate::theme::{ThemeMetadata, Colors, Controls, Styles, StyleConfig};
use crate::events::{Event, KeyEvent, MouseEvent, Action, AppAction};
use ratatui::{
    backend::TestBackend,
    Terminal,
    layout::Rect,
};

// Mock component for testing Component trait implementation
struct MockComponent {
    state: ComponentState,
}

impl Component for MockComponent {
    fn new() -> Self {
        Self {
            state: ComponentState::default(),
        }
    }

    fn render(&self, _frame: &mut Frame, _area: Rect, _focused: bool, _theme: &Theme) {
        // Mock render implementation
    }

    fn update(&mut self, action: Action) -> Option<Action> {
        // Mock update implementation - echo back the action
        Some(action)
    }

    fn handle_event(&mut self, event: Event) -> Option<Action> {
        // Mock event handling - convert events to actions for testing
        match event {
            Event::Key(_) => Some(Action::App(AppAction::Quit)),
            _ => None,
        }
    }

    fn focused(&self) -> bool {
        self.state.focused
    }

    fn set_focused(&mut self, focused: bool) {
        self.state.focused = focused;
    }
}

// Helper function to create a minimal theme for testing
fn create_test_theme() -> Theme {
    Theme {
        metadata: ThemeMetadata {
            name: String::from("Test Theme"),
            author: String::from("Test Author"),
            version: String::from("1.0"),
        },
        colors: Colors {
            primary: String::from("#F92672"),
            secondary: String::from("#A6E22E"),
            background: String::from("#272822"),
            foreground: String::from("#F8F8F2"),
            active: String::from("#FD971F"),
            inactive: String::from("#75715E"),
            playing: String::from("#66D9EF"),
            error: String::from("#F92672"),
        },
        controls: Controls {
            record: String::from("⏺"),
            play: String::from("⏵"),
            rewind: String::from("◀◀"),
            fast_forward: String::from("⏵⏵"),
            stop: String::from("⏏"),
            pause: String::from("⏸"),
            next: String::from("⬇"),
            previous: String::from("⬆"),
        },
        styles: Styles {
            border_focused: StyleConfig {
                fg: Some(String::from("#F92672")),
                bg: None,
                modifiers: Some(vec![String::from("BOLD")]),
            },
            border_unfocused: StyleConfig {
                fg: Some(String::from("#A6E22E")),
                bg: None,
                modifiers: None,
            },
            text_normal: StyleConfig {
                fg: Some(String::from("#F8F8F2")),
                bg: None,
                modifiers: None,
            },
            text_bold: StyleConfig {
                fg: Some(String::from("#F8F8F2")),
                bg: None,
                modifiers: Some(vec![String::from("BOLD")]),
            },
            text_dim: StyleConfig {
                fg: Some(String::from("#75715E")),
                bg: None,
                modifiers: None,
            },
            button: StyleConfig {
                fg: Some(String::from("#A6E22E")),
                bg: None,
                modifiers: Some(vec![String::from("BOLD")]),
            },
            list_item: StyleConfig {
                fg: Some(String::from("#F8F8F2")),
                bg: None,
                modifiers: None,
            },
            list_selected: StyleConfig {
                fg: Some(String::from("#272822")),
                bg: Some(String::from("#FD971F")),
                modifiers: Some(vec![String::from("BOLD")]),
            },
            playing_item: StyleConfig {
                fg: Some(String::from("#66D9EF")),
                bg: None,
                modifiers: Some(vec![String::from("BOLD")]),
            },
            progress_bar: StyleConfig {
                fg: Some(String::from("#FD971F")),
                bg: Some(String::from("#272822")),
                modifiers: None,
            },
            volume_indicator: StyleConfig {
                fg: Some(String::from("#66D9EF")),
                bg: None,
                modifiers: Some(vec![String::from("BOLD")]),
            },
            tab_active: StyleConfig {
                fg: Some(String::from("#272822")),
                bg: Some(String::from("#F92672")),
                modifiers: Some(vec![String::from("BOLD")]),
            },
            tab_inactive: StyleConfig {
                fg: Some(String::from("#F92672")),
                bg: None,
                modifiers: None,
            },
        },
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_component_state_default() {
        let state = ComponentState::default();
        assert!(!state.focused, "ComponentState should not be focused by default");
    }

    #[test]
    fn test_component_state_clone() {
        let state = ComponentState { focused: true };
        let cloned = state.clone();
        assert_eq!(state.focused, cloned.focused, "Cloned state should match original");
    }

    #[test]
    fn test_mock_component_creation() {
        let component = MockComponent::new();
        assert!(!component.focused(), "New component should not be focused by default");
    }

    #[test]
    fn test_mock_component_focus() {
        let mut component = MockComponent::new();
        assert!(!component.focused(), "Should start unfocused");
        
        component.set_focused(true);
        assert!(component.focused(), "Should be focused after set_focused(true)");
        
        component.set_focused(false);
        assert!(!component.focused(), "Should be unfocused after set_focused(false)");
    }

    #[test]
    fn test_mock_component_event_handling() {
        let mut component = MockComponent::new();
        
        // Test key event
        let key_event = Event::Key(KeyEvent::Char('q'));
        let action = component.handle_event(key_event);
        assert!(matches!(action, Some(Action::App(AppAction::Quit))), 
            "Key event should produce Quit action");

        // Test mouse event
        let mouse_event = Event::Mouse(MouseEvent::Click { x: 0, y: 0 });
        let action = component.handle_event(mouse_event);
        assert!(action.is_none(), "Mouse event should produce no action");
    }

    #[test]
    fn test_mock_component_update() {
        let mut component = MockComponent::new();
        let action = Action::App(AppAction::Quit);
        let result = component.update(action.clone());
        assert!(matches!(result, Some(Action::App(AppAction::Quit))), 
            "Update should echo back the action");
    }

    #[test]
    fn test_create_block() {
        let theme = create_test_theme();
        let backend = TestBackend::new(20, 3);
        let mut terminal = Terminal::new(backend).unwrap();
        
        // Test unfocused block
        terminal.draw(|frame| {
            let block = create_block("Test", false, &theme);
            frame.render_widget(block, Rect::new(0, 0, 20, 3));
        }).unwrap();
        
        let buffer = terminal.backend().buffer();
        let content = buffer.content.iter()
            .map(|cell| cell.symbol.clone())
            .collect::<String>();
        assert!(content.contains("─"), "Unfocused block should have normal borders");
        assert!(content.contains("Test"), "Should display title");
        
        // Test focused block
        terminal.draw(|frame| {
            let block = create_block("Test", true, &theme);
            frame.render_widget(block, Rect::new(0, 0, 20, 3));
        }).unwrap();
        
        let buffer = terminal.backend().buffer();
        let content = buffer.content.iter()
            .map(|cell| cell.symbol.clone())
            .collect::<String>();
        assert!(content.contains("━"), "Focused block should have thick borders");
        assert!(content.contains("Test"), "Should display title when focused");
    }
}
