use crate::components::{Component, ComponentState};
use crate::events::{Event, Action};
use crate::theme::Theme;
use ratatui::prelude::*;

/// Mock component for testing component behavior
pub struct MockComponent {
    state: ComponentState,
    pub last_event: Option<Event>,
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::events::KeyEvent;

    #[test]
    fn test_mock_component_creation() {
        let component = MockComponent::new();
        assert!(!component.focused());
        assert_eq!(component.last_event, None);
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
}
