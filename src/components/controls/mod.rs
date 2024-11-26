use ratatui::prelude::*;
use std::cell::RefCell;
use crate::events::{Event, Action};
use crate::theme::Theme;
use super::{Component, ComponentState};

#[derive(Clone)]
pub struct Controls {
    pub(crate) state: ComponentState,
    pub(crate) is_playing: bool,
    pub(crate) is_recording: bool,
    pub(crate) is_seeking_forward: bool,
    pub(crate) is_seeking_backward: bool,
    pub(crate) focused_button: usize,
    pub(crate) focused_section: Section,
    pub(crate) area: RefCell<Option<Rect>>,
}

#[derive(Clone, PartialEq)]
pub(crate) enum Section {
    Controls,
    Volume,
}

mod layout;
mod events;
mod actions;

impl Component for Controls {
    fn new() -> Self {
        Controls {
            state: ComponentState::default(),
            is_playing: false,
            is_recording: false,
            is_seeking_forward: false,
            is_seeking_backward: false,
            focused_button: 0,
            focused_section: Section::Controls,
            area: RefCell::new(None),
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, focused: bool, theme: &Theme) {
        layout::render(self, frame, area, focused, theme);
    }

    fn update(&mut self, action: Action) -> Option<Action> {
        actions::update(self, action)
    }

    fn focused(&self) -> bool {
        self.state.focused
    }

    fn set_focused(&mut self, focused: bool) {
        self.state.focused = focused;
    }

    fn handle_event(&mut self, event: Event) -> Option<Action> {
        events::handle_event(self, event)
    }
}
