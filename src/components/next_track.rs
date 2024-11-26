use ratatui::prelude::*;
use super::{Component, ComponentState, create_block};
use crate::events::{Event, Action, KeyEvent};
use crate::theme::Theme;

#[derive(Clone)]
pub struct NextTrack {
    state: ComponentState,
}

impl Component for NextTrack {
    fn new() -> Self {
        Self {
            state: ComponentState::default(),
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, focused: bool, theme: &Theme) {
        let block = create_block("Next", focused, theme);
        frame.render_widget(block, area);
    }

    fn update(&mut self, _action: Action) -> Option<Action> {
        None
    }

    fn focused(&self) -> bool {
        self.state.focused
    }

    fn set_focused(&mut self, focused: bool) {
        self.state.focused = focused;
    }

    fn handle_event(&mut self, event: Event) -> Option<Action> {
        if self.focused() {
            match event {
                Event::Key(KeyEvent::Enter) => Some(Action::NextTrack),
                _ => None,
            }
        } else {
            None
        }
    }
}
