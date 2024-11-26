use ratatui::prelude::*;
use crate::components::{Component, ComponentState, create_block};
use crate::events::{Event, Action, KeyEvent, MouseEvent};
use crate::theme::Theme;
use std::cell::RefCell;

#[cfg(test)]
mod tests;

pub struct VolumeControl {
    state: ComponentState,
    volume: u8,
    area: RefCell<Option<Rect>>,
}

// Manual Clone implementation to properly handle RefCell
impl Clone for VolumeControl {
    fn clone(&self) -> Self {
        Self {
            state: self.state.clone(),
            volume: self.volume,
            area: RefCell::new(*self.area.borrow()),
        }
    }
}

impl Component for VolumeControl {
    fn new() -> Self {
        Self {
            state: ComponentState::default(),
            volume: 50, // Default volume 50%
            area: RefCell::new(None),
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, focused: bool, theme: &Theme) {
        let title = format!("Volume: {}%", self.volume);
        let block = create_block(title.as_str(), focused, theme);
        
        // Store the area for mouse interaction calculations
        *self.area.borrow_mut() = Some(area);
        
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
        if !self.focused() {
            return None;
        }

        match event {
            Event::Key(KeyEvent::Up) => Some(Action::VolumeUp),
            Event::Key(KeyEvent::Down) => Some(Action::VolumeDown),
            Event::Mouse(mouse_event) => self.handle_mouse_event(mouse_event),
            _ => None,
        }
    }
}

impl VolumeControl {
    fn handle_mouse_event(&self, event: MouseEvent) -> Option<Action> {
        match event {
            MouseEvent::Click { x, y } => {
                // Check if click is within our area
                if let Some(area) = *self.area.borrow() {
                    if x >= area.x && x < area.x + area.width &&
                       y >= area.y && y < area.y + area.height {
                        // Calculate volume based on click position
                        let relative_x = x - area.x;
                        // If clicked at the rightmost position, set to 100%
                        let volume = if relative_x == area.width - 1 {
                            100
                        } else {
                            ((relative_x as f32 / (area.width - 1) as f32) * 100.0).round() as u8
                        };
                        Some(Action::SetVolume(volume.min(100)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            },
            MouseEvent::Scroll { delta } => {
                if delta > 0 {
                    Some(Action::VolumeDown)
                } else {
                    Some(Action::VolumeUp)
                }
            }
        }
    }
}

#[cfg(test)]
mod volume_tests {
    use super::*;

    #[test]
    fn test_mouse_scroll() {
        let mut control = VolumeControl::new();
        control.set_focused(true);
        
        // Test scroll up (volume down)
        let result = control.handle_event(Event::Mouse(MouseEvent::Scroll { delta: 1 }));
        assert_eq!(result, Some(Action::VolumeDown));
        
        // Test scroll down (volume up)
        let result = control.handle_event(Event::Mouse(MouseEvent::Scroll { delta: -1 }));
        assert_eq!(result, Some(Action::VolumeUp));
    }

    #[test]
    fn test_mouse_click() {
        let mut control = VolumeControl::new();
        control.set_focused(true);
        
        // Set a test area (width of 101 to have positions 0-100)
        *control.area.borrow_mut() = Some(Rect::new(0, 0, 101, 1));
        
        // Test click at 50%
        let result = control.handle_event(Event::Mouse(MouseEvent::Click { x: 50, y: 0 }));
        if let Some(Action::SetVolume(vol)) = result {
            assert!((49..=51).contains(&vol), "Volume should be approximately 50%");
        } else {
            panic!("Expected SetVolume action");
        }
        
        // Test click at max (last position)
        let result = control.handle_event(Event::Mouse(MouseEvent::Click { x: 100, y: 0 }));
        assert_eq!(result, Some(Action::SetVolume(100)));
        
        // Test click outside area
        let result = control.handle_event(Event::Mouse(MouseEvent::Click { x: 150, y: 0 }));
        assert_eq!(result, None);
    }

    #[test]
    fn test_unfocused_events() {
        let mut control = VolumeControl::new();
        control.set_focused(false);
        
        // Test that unfocused component ignores events
        assert_eq!(control.handle_event(Event::Mouse(MouseEvent::Scroll { delta: 1 })), None);
        assert_eq!(control.handle_event(Event::Mouse(MouseEvent::Click { x: 50, y: 0 })), None);
    }
}
