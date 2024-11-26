use ratatui::prelude::*;
use super::{Component, ComponentState, create_block};
use crate::events::{Event, Action};
use crate::theme::Theme;

#[derive(Clone, PartialEq)]
enum PlaybackState {
    Stopped,
    Playing,
    Paused,
}

#[derive(Clone)]
pub struct PlaybackStatus {
    state: ComponentState,
    playback_state: PlaybackState,
}

impl Component for PlaybackStatus {
    fn new() -> Self {
        Self {
            state: ComponentState::default(),
            playback_state: PlaybackState::Stopped,
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, focused: bool, theme: &Theme) {
        let status = match self.playback_state {
            PlaybackState::Stopped => "Stopped",
            PlaybackState::Playing => "Playing",
            PlaybackState::Paused => "Paused",
        };
        let title = format!("Status: {}", status);
        let block = create_block(title.as_str(), focused, theme);
        frame.render_widget(block, area);
    }

    fn update(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::Play => {
                self.playback_state = PlaybackState::Playing;
                None
            }
            Action::Pause => {
                self.playback_state = PlaybackState::Paused;
                None
            }
            Action::Stop => {
                self.playback_state = PlaybackState::Stopped;
                None
            }
            _ => None,
        }
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
