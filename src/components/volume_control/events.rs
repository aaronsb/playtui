use crate::events::{Event, Action, KeyEvent, MouseEvent};
use super::state::VolumeState;

pub fn handle_event(state: &mut VolumeState, event: Event, focused: bool) -> Option<Action> {
    if !focused {
        return None;
    }

    match event {
        Event::Key(KeyEvent::Up) => Some(Action::VolumeUp),
        Event::Key(KeyEvent::Down) => Some(Action::VolumeDown),
        Event::Mouse(mouse_event) => handle_mouse_event(state, mouse_event),
        _ => None,
    }
}

fn handle_mouse_event(state: &VolumeState, event: MouseEvent) -> Option<Action> {
    match event {
        MouseEvent::Click { x, y } => {
            // Check if click is within our area
            if let Some(area) = state.get_area() {
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

#[cfg(test)]
mod tests {
    use super::*;
    use ratatui::prelude::*;

    #[test]
    fn test_mouse_scroll() {
        let mut state = VolumeState::default();
        
        // Test scroll up (volume down)
        let result = handle_event(&mut state, Event::Mouse(MouseEvent::Scroll { delta: 1 }), true);
        assert_eq!(result, Some(Action::VolumeDown));
        
        // Test scroll down (volume up)
        let result = handle_event(&mut state, Event::Mouse(MouseEvent::Scroll { delta: -1 }), true);
        assert_eq!(result, Some(Action::VolumeUp));
    }

    #[test]
    fn test_mouse_click() {
        let mut state = VolumeState::default();
        
        // Set a test area (width of 101 to have positions 0-100)
        state.set_area(Rect::new(0, 0, 101, 1));
        
        // Test click at 50%
        let result = handle_event(&mut state, Event::Mouse(MouseEvent::Click { x: 50, y: 0 }), true);
        if let Some(Action::SetVolume(vol)) = result {
            assert!((49..=51).contains(&vol), "Volume should be approximately 50%");
        } else {
            panic!("Expected SetVolume action");
        }
        
        // Test click at max (last position)
        let result = handle_event(&mut state, Event::Mouse(MouseEvent::Click { x: 100, y: 0 }), true);
        assert_eq!(result, Some(Action::SetVolume(100)));
        
        // Test click outside area
        let result = handle_event(&mut state, Event::Mouse(MouseEvent::Click { x: 150, y: 0 }), true);
        assert_eq!(result, None);
    }

    #[test]
    fn test_unfocused_events() {
        let mut state = VolumeState::default();
        
        // Test that unfocused component ignores events
        assert_eq!(handle_event(&mut state, Event::Mouse(MouseEvent::Scroll { delta: 1 }), false), None);
        assert_eq!(handle_event(&mut state, Event::Mouse(MouseEvent::Click { x: 50, y: 0 }), false), None);
    }
}
