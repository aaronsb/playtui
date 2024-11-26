use ratatui::{
    prelude::*,
    widgets::{Block, Borders, BorderType, Paragraph},
    layout::{Layout, Direction, Constraint},
};

use crate::events::{Event, Action, KeyEvent, MouseEvent, PlayerAction};
use super::{Component, ComponentState};
use crate::theme::Theme;
use std::cell::RefCell;

#[derive(Clone)]
pub struct Controls {
    state: ComponentState,
    is_playing: bool,
    is_recording: bool,
    is_seeking_forward: bool,
    is_seeking_backward: bool,
    focused_button: usize,  // Track which button is focused
    focused_section: Section, // Track whether controls or volume is focused
    area: RefCell<Option<Rect>>,    // Store the component's area for mouse hit testing
}

#[derive(Clone, PartialEq)]
enum Section {
    Controls,
    Volume,
}

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
        // Store the area for mouse hit testing
        *self.area.borrow_mut() = Some(area);

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
            .border_type(if focused && self.focused_section == Section::Controls { 
                BorderType::Thick 
            } else { 
                BorderType::Rounded 
            })
            .border_style(if focused && self.focused_section == Section::Controls {
                theme.get_style("border_focused")
            } else {
                theme.get_style("border_unfocused")
            });

        let inner_controls = controls_block.inner(controls_area);
        frame.render_widget(controls_block, controls_area);

        // Create control buttons with themed icons and states
        let controls = vec![
            (self.is_recording, &theme.controls.record, "Record"),
            (!self.is_playing, &theme.controls.play, "Play"),
            (self.is_seeking_backward, &theme.controls.rewind, "Rew"),
            (self.is_seeking_forward, &theme.controls.fast_forward, "FF"),
            (false, &theme.controls.stop, "Stop/Eject"),
            (self.is_playing, &theme.controls.pause, "Pause"),
            (false, &theme.controls.next, "Next"),
            (false, &theme.controls.previous, "Prev"),
        ];

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

            // Determine button style based on state
            let button_style = if focused && 
                             self.focused_section == Section::Controls && 
                             self.focused_button == i {
                // Button is focused
                theme.get_style("list_selected")
            } else if *active && *label == "Record" {
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
            .border_type(if focused && self.focused_section == Section::Volume { 
                BorderType::Thick 
            } else { 
                BorderType::Rounded 
            })
            .border_style(if focused && self.focused_section == Section::Volume {
                theme.get_style("border_focused")
            } else {
                theme.get_style("border_unfocused")
            });

        let inner_volume = volume_block.inner(volume_area);
        frame.render_widget(volume_block, volume_area);

        // TODO: Implement volume slider widget
        frame.render_widget(
            Paragraph::new("Volume Slider")
                .alignment(Alignment::Center)
                .style(if focused && self.focused_section == Section::Volume {
                    theme.get_style("list_selected")
                } else {
                    theme.get_style("text_normal")
                }),
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
            KeyEvent::Tab => {
                if self.focused_section == Section::Controls {
                    // Move to next button or switch to volume section
                    self.focused_button = (self.focused_button + 1) % 8;
                    if self.focused_button == 0 {
                        self.focused_section = Section::Volume;
                    }
                } else {
                    // Switch back to controls section
                    self.focused_section = Section::Controls;
                    self.focused_button = 0;
                }
                None
            }
            KeyEvent::BackTab => {
                if self.focused_section == Section::Controls {
                    // Move to previous button or switch to volume section
                    if self.focused_button == 0 {
                        self.focused_section = Section::Volume;
                    } else {
                        self.focused_button = (self.focused_button - 1) % 8;
                    }
                } else {
                    // Switch back to controls section
                    self.focused_section = Section::Controls;
                    self.focused_button = 7;
                }
                None
            }
            KeyEvent::Enter => {
                // Trigger action based on focused button
                match self.focused_section {
                    Section::Controls => match self.focused_button {
                        0 => Some(Action::Player(PlayerAction::Record)),
                        1 => Some(Action::Player(PlayerAction::Play)),
                        2 => Some(Action::Player(PlayerAction::Rewind)),
                        3 => Some(Action::Player(PlayerAction::FastForward)),
                        4 => Some(Action::Player(PlayerAction::Stop)),
                        5 => Some(Action::Player(PlayerAction::Pause)),
                        6 => Some(Action::Player(PlayerAction::LoadTrack(String::new()))), // Next
                        7 => Some(Action::Player(PlayerAction::LoadTrack(String::new()))), // Previous
                        _ => None,
                    },
                    Section::Volume => None, // TODO: Implement volume control
                }
            }
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
                // First get and validate the area
                let area = match *self.area.borrow() {
                    Some(area) if Self::is_point_in_rect(x, y, area) => area,
                    _ => return None,
                };

                // Calculate section and button information
                let controls_width = (area.width * 8) / 10; // 80% of total width
                let relative_x = x - area.x;

                // Drop the area borrow before proceeding with state changes
                drop(self.area.borrow());

                if relative_x <= controls_width {
                    // Click in controls section
                    self.focused_section = Section::Controls;
                    
                    // Calculate button width and determine which button was clicked
                    let button_width = controls_width / 8;
                    let clicked_button = (relative_x / button_width) as usize;
                    if clicked_button < 8 {
                        self.focused_button = clicked_button;
                        // Now safe to call handle_key_event
                        self.handle_key_event(KeyEvent::Enter)
                    } else {
                        None
                    }
                } else {
                    // Click in volume section
                    self.focused_section = Section::Volume;
                    // TODO: Implement volume control click handling
                    None
                }
            }
            _ => None,
        }
    }

    fn is_point_in_rect(x: u16, y: u16, rect: Rect) -> bool {
        x >= rect.x && x < rect.x + rect.width &&
        y >= rect.y && y < rect.y + rect.height
    }
}
