use ratatui::prelude::*;
use super::{Component, ComponentState, create_block};
use crate::events::{Event, Action, KeyEvent};

#[derive(Clone)]
pub struct PlayPause {
    state: ComponentState,
    playing: bool,
}

impl Component for PlayPause {
    fn new() -> Self {
        Self {
            state: ComponentState::default(),
            playing: false,
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, focused: bool) {
        let title = if self.playing { "Pause" } else { "Play" };
        let block = create_block(title, focused);
        frame.render_widget(block, area);
    }

    fn update(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::Play => {
                self.playing = true;
                None
            }
            Action::Pause => {
                self.playing = false;
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
                Event::Key(KeyEvent::Enter) => {
                    if self.playing {
                        Some(Action::Pause)
                    } else {
                        Some(Action::Play)
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    }
}
