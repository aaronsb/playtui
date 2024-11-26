use ratatui::{prelude::*, widgets::ListState};
use crate::events::{Event, Action};
use crate::components::{Component, ComponentState};
use crate::theme::Theme;

mod event_handler;
mod selection;
mod renderer;

#[derive(Clone)]
pub struct Playlist {
    pub(crate) state: ComponentState,
    pub(crate) list_state: ListState,
    pub(crate) tracks: Vec<String>,
    pub(crate) scroll_offset: usize,
}

impl Component for Playlist {
    fn new() -> Self {
        Playlist {
            state: ComponentState::default(),
            list_state: ListState::default(),
            tracks: Vec::new(),
            scroll_offset: 0,
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, focused: bool, theme: &Theme) {
        renderer::render(self, frame, area, focused, theme);
    }

    fn update(&mut self, action: Action) -> Option<Action> {
        selection::update(self, action)
    }

    fn handle_event(&mut self, event: Event) -> Option<Action> {
        if !self.state.focused {
            return None;
        }
        event_handler::handle_event_internal(self, event)
    }

    fn focused(&self) -> bool {
        self.state.focused
    }

    fn set_focused(&mut self, focused: bool) {
        self.state.focused = focused;
    }
}

// Re-export internal functions for use within the module
pub(self) use selection::{select_next, select_previous};
