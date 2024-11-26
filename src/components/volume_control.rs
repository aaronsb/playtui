use ratatui::prelude::*;
use super::{Component, ComponentState, create_block};
use crate::events::{Event, Action, KeyEvent};

#[derive(Clone)]
pub struct VolumeControl {
    state: ComponentState,
    volume: u8,
}

impl Component for VolumeControl {
    fn new() -> Self {
        Self {
            state: ComponentState::default(),
            volume: 50, // Default volume 50%
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, focused: bool) {
        let title = format!("Volume: {}%", self.volume);
        let block = create_block(title.as_str(), focused);
        frame.render_widget(block, area);
    }

    fn update(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::VolumeUp => {
                if self.volume < 100 {
                    self.volume = self.volume.saturating_add(5);
                }
                Some(Action::SetVolume(self.volume))
            }
            Action::VolumeDown => {
                if self.volume > 0 {
                    self.volume = self.volume.saturating_sub(5);
                }
                Some(Action::SetVolume(self.volume))
            }
            Action::SetVolume(vol) => {
                self.volume = vol;
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

    fn handle_event(&mut self, event: Event) -> Option<Action> {
        if self.focused() {
            match event {
                Event::Key(KeyEvent::Up) => Some(Action::VolumeUp),
                Event::Key(KeyEvent::Down) => Some(Action::VolumeDown),
                _ => None,
            }
        } else {
            None
        }
    }
}
