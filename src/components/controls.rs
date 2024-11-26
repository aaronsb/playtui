use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
    layout::{Layout, Direction, Constraint},
};

use crate::events::{Event, Action, KeyEvent, MouseEvent, PlayerAction};
use super::{Component, ComponentState};
use crate::theme::Theme;

#[derive(Clone)]
pub struct Controls {
    state: ComponentState,
    is_playing: bool,
    is_recording: bool,
    is_seeking_forward: bool,
    is_seeking_backward: bool,
}

impl Component for Controls {
    fn new() -> Self {
        Controls {
            state: ComponentState::default(),
            is_playing: false,
            is_recording: false,
            is_seeking_forward: false,
            is_seeking_backward: false,
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, focused: bool, theme: &Theme) {
        // Split into controls (80%) and volume (20%) frames
        let chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(80),
                Constraint::Percentage(20),
            ])
            .split(area);

        let controls_area = chunks[0];
        let volume_area = chunks[1];

        // Render controls frame
        let controls_block = Block::default()
            .title("Controls")
            .borders(Borders::ALL)
            .border_style(if focused {
                theme.get_style("border_focused")
            } else {
                theme.get_style("border_unfocused")
            });

        let inner_controls = controls_block.inner(controls_area);
        frame.render_widget(controls_block, controls_area);

        // Create control buttons with nerdfont icons and states
        let controls = vec![
            (self.is_recording, "⏺", "Record"),      // Record (red circle)
            (!self.is_playing, "⏵", "Play"),         // Play
            (self.is_seeking_backward, "◀◀", "Rew"),  // Rewind (double left arrow)
            (self.is_seeking_forward, "⏵⏵", "FF"),    // Fast Forward (double play)
            (false, "⏏", "Stop/Eject"),             // Stop/Eject
            (self.is_playing, "⏸", "Pause"),         // Pause
            (false, "⬇", "Next"),                   // Next Track (down arrow)
            (false, "⬆", "Prev"),                   // Previous Track (up arrow)
        ];

        // Calculate button width including padding
        let button_width = inner_controls.width / controls.len() as u16;
        let button_padding = 2;

        // Render each button with highlight and shadow effects
        for (i, (active, icon, label)) in controls.iter().enumerate() {
            let x = inner_controls.x + (i as u16 * button_width);
            let button_area = Rect::new(x, inner_controls.y, button_width, inner_controls.height);

            // Button background with shadow effect
            let shadow_style = theme.get_style("button_shadow");
            let shadow_area = Rect::new(x + 1, button_area.y + 1, button_width - 1, button_area.height - 1);
            frame.render_widget(Block::default().style(shadow_style), shadow_area);

            // Special color for record button when active
            let button_style = if *active && *label == "Record" {
                theme.get_style("record_button_active")
            } else if *active {
                theme.get_style("button_active")
            } else {
                theme.get_style("button")
            };

            let button_text = format!("{} {}", icon, label);
            frame.render_widget(
                Paragraph::new(button_text)
                    .alignment(Alignment::Center)
                    .style(button_style),
                button_area,
            );
        }

        // Render volume frame
        let volume_block = Block::default()
            .title("Volume")
            .borders(Borders::ALL)
            .border_style(theme.get_style("border_unfocused"));

        let inner_volume = volume_block.inner(volume_area);
        frame.render_widget(volume_block, volume_area);

        // TODO: Implement volume slider widget
        frame.render_widget(
            Paragraph::new("Volume Slider")
                .alignment(Alignment::Center)
                .style(theme.get_style("text_normal")),
            inner_volume,
        );
    }

    fn update(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::Player(PlayerAction::Play) => {
                self.is_playing = true;
                self.is_seeking_forward = false;
                self.is_seeking_backward = false;
                None
            }
            Action::Player(PlayerAction::Pause) => {
                self.is_playing = false;
                None
            }
            Action::Player(PlayerAction::Stop) => {
                self.is_playing = false;
                self.is_seeking_forward = false;
                self.is_seeking_backward = false;
                self.is_recording = false;
                None
            }
            Action::Player(PlayerAction::FastForward) => {
                self.is_seeking_forward = true;
                self.is_seeking_backward = false;
                None
            }
            Action::Player(PlayerAction::Rewind) => {
                self.is_seeking_backward = true;
                self.is_seeking_forward = false;
                None
            }
            Action::Player(PlayerAction::Record) => {
                self.is_recording = !self.is_recording;
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
            KeyEvent::Play => Some(Action::Player(PlayerAction::Play)),
            KeyEvent::Pause => Some(Action::Player(PlayerAction::Pause)),
            KeyEvent::Stop => Some(Action::Player(PlayerAction::Stop)),
            KeyEvent::FastForward => Some(Action::Player(PlayerAction::FastForward)),
            KeyEvent::Rewind => Some(Action::Player(PlayerAction::Rewind)),
            KeyEvent::Record => Some(Action::Player(PlayerAction::Record)),
            KeyEvent::Next => Some(Action::Player(PlayerAction::LoadTrack(String::new()))),
            KeyEvent::Previous => Some(Action::Player(PlayerAction::LoadTrack(String::new()))),
            KeyEvent::VolumeUp => Some(Action::Player(PlayerAction::SetVolume(100))),
            KeyEvent::VolumeDown => Some(Action::Player(PlayerAction::SetVolume(0))),
            _ => None,
        }
    }

    fn handle_mouse_event(&mut self, event: MouseEvent) -> Option<Action> {
        match event {
            MouseEvent::Click { x, y } => {
                // Calculate button positions and check if click falls within any button area
                // TODO: Implement proper click handling for control buttons
                None
            }
            _ => None,
        }
    }
}
