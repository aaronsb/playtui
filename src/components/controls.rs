use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use crate::events::{Event, Action, KeyEvent, MouseEvent, PlayerAction};
use super::{Component, ComponentState};

#[derive(Clone)]
pub struct Controls {
    state: ComponentState,
    is_playing: bool,
}

impl Component for Controls {
    fn new() -> Self {
        Controls {
            state: ComponentState::default(),
            is_playing: false,
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, focused: bool) {
        let block = Block::default()
            .title("Controls")
            .borders(Borders::ALL)
            .border_style(if focused {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            });

        let inner_area = block.inner(area);

        // Render the block first
        frame.render_widget(block, area);

        // Create controls text with symbols
        let controls_text = vec![
            if self.is_playing { "⏸ Pause" } else { "▶ Play" },
            "⏹ Stop",
            "⏮ Previous",
            "⏭ Next",
            "- Vol Down",
            "+ Vol Up",
        ];

        let controls_display = controls_text.join(" | ");

        frame.render_widget(
            Paragraph::new(controls_display)
                .alignment(Alignment::Center),
            inner_area,
        );
    }

    fn update(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::Player(PlayerAction::Play) => {
                self.is_playing = true;
                None
            }
            Action::Player(PlayerAction::Pause) => {
                self.is_playing = false;
                None
            }
            Action::Player(PlayerAction::Stop) => {
                self.is_playing = false;
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
        if !self.state.focused {
            return None;
        }

        match event {
            Event::Key(key_event) => self.handle_key_event(key_event),
            Event::Mouse(mouse_event) => self.handle_mouse_event(mouse_event),
            _ => None,
        }
    }
}

impl Controls {
    fn handle_key_event(&mut self, event: KeyEvent) -> Option<Action> {
        match event {
            KeyEvent::Play => {
                if self.is_playing {
                    Some(Action::Player(PlayerAction::Pause))
                } else {
                    Some(Action::Player(PlayerAction::Play))
                }
            }
            KeyEvent::Stop => Some(Action::Player(PlayerAction::Stop)),
            KeyEvent::Next => Some(Action::Player(PlayerAction::LoadTrack(String::new()))), // Actual track loading handled by playlist
            KeyEvent::Previous => Some(Action::Player(PlayerAction::LoadTrack(String::new()))), // Actual track loading handled by playlist
            KeyEvent::VolumeUp => Some(Action::Player(PlayerAction::SetVolume(100))), // TODO: Implement proper volume adjustment
            KeyEvent::VolumeDown => Some(Action::Player(PlayerAction::SetVolume(0))), // TODO: Implement proper volume adjustment
            _ => None,
        }
    }

    fn handle_mouse_event(&mut self, event: MouseEvent) -> Option<Action> {
        match event {
            MouseEvent::Click { x, y } => {
                // TODO: Implement click handling for control buttons
                // This would require calculating button positions and checking if the click
                // falls within any of them
                None
            }
            _ => None,
        }
    }
}
