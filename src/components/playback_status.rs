use ratatui::prelude::*;
use super::{Component, ComponentState, create_block};
use crate::events::{Event, Action};

#[derive(Clone)]
pub struct PlaybackStatus {
    state: ComponentState,
}

impl Component for PlaybackStatus {
    fn new() -> Self {
        Self {
            state: ComponentState::default(),
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, focused: bool) {
        let block = create_block("Playback Status", focused);
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

    fn handle_event(&mut self, _event: Event) -> Option<Action> {
        None
    }
}
