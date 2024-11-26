use ratatui::prelude::*;
use super::{Component, ComponentState, create_block};
use crate::events::{Event, Action};
use crate::theme::Theme;

mod state;
mod view;
mod actions;

#[cfg(test)]
mod tests;

use state::TrackListState;

#[derive(Clone)]
pub struct TrackList {
    state: TrackListState,
}

impl Component for TrackList {
    fn new() -> Self {
        Self {
            state: TrackListState::default(),
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, focused: bool, theme: &Theme) {
        // Update focused state before rendering
        let mut state = self.state.clone();
        state.set_focused(focused);
        view::render(&state, frame, area, theme);
    }

    fn update(&mut self, action: Action) -> Option<Action> {
        actions::handle_action(&mut self.state, action)
    }

    fn focused(&self) -> bool {
        self.state.focused()
    }

    fn set_focused(&mut self, focused: bool) {
        self.state.set_focused(focused);
    }

    fn handle_event(&mut self, event: Event) -> Option<Action> {
        actions::handle_event(&mut self.state, event)
    }
}
