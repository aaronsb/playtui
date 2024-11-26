use crate::events::{Event, Action, KeyEvent, PlayerAction, MouseEvent, NavigationEvent};
use crate::components::Component;
use super::{Controls, Section};

pub fn handle_event(controls: &mut Controls, event: Event) -> Option<Action> {
    match event {
        Event::Key(key_event) => match key_event {
            // Navigation key events should work regardless of focus
            KeyEvent::Left => Some(Action::NavigateLeft),
            KeyEvent::Right => Some(Action::NavigateRight),
            KeyEvent::Up => Some(Action::NavigateUp),
            KeyEvent::Down => Some(Action::NavigateDown),
            // Other key events require focus
            _ => {
                if !controls.focused() {
                    return None;
                }
                match key_event {
                    KeyEvent::Tab => {
                        handle_tab_navigation(controls, false);
                        Some(Action::Refresh)
                    }
                    KeyEvent::BackTab => {
                        handle_tab_navigation(controls, true);
                        Some(Action::Refresh)
                    }
                    KeyEvent::Enter => handle_enter_action(controls),
                    // Handle media control keys
                    KeyEvent::Play => Some(Action::Player(PlayerAction::Play)),
                    KeyEvent::Pause => Some(Action::Player(PlayerAction::Pause)),
                    KeyEvent::Stop => Some(Action::Player(PlayerAction::Stop)),
                    KeyEvent::FastForward => Some(Action::Player(PlayerAction::FastForward)),
                    KeyEvent::Rewind => Some(Action::Player(PlayerAction::Rewind)),
                    KeyEvent::Record => Some(Action::Player(PlayerAction::Record)),
                    KeyEvent::Next => Some(Action::Player(PlayerAction::LoadTrack(String::new()))),
                    KeyEvent::Previous => Some(Action::Player(PlayerAction::LoadTrack(String::new()))),
                    _ => None,
                }
            }
        },
        Event::Mouse(mouse_event) => {
            if !controls.focused() {
                return None;
            }
            handle_mouse_event(controls, mouse_event)
        },
        Event::Navigation(nav_event) => handle_navigation(controls, nav_event),
        Event::System(_) => None,
    }
}

fn handle_navigation(controls: &mut Controls, event: NavigationEvent) -> Option<Action> {
    match event {
        NavigationEvent::Left => {
            if controls.focused_section == Section::Controls {
                // Move to previous button
                if controls.focused_button == 0 {
                    controls.focused_button = 7;
                } else {
                    controls.focused_button = (controls.focused_button - 1) % 8;
                }
                Some(Action::Refresh)
            } else {
                None
            }
        },
        NavigationEvent::Right => {
            if controls.focused_section == Section::Controls {
                // Move to next button
                controls.focused_button = (controls.focused_button + 1) % 8;
                Some(Action::Refresh)
            } else {
                None
            }
        },
        NavigationEvent::Up => {
            if controls.focused_section == Section::Volume {
                controls.focused_section = Section::Controls;
                Some(Action::Refresh)
            } else {
                None
            }
        },
        NavigationEvent::Down => {
            if controls.focused_section == Section::Controls {
                controls.focused_section = Section::Volume;
                Some(Action::Refresh)
            } else {
                None
            }
        },
    }
}

fn handle_tab_navigation(controls: &mut Controls, backward: bool) {
    if controls.focused_section == Section::Controls {
        if backward {
            // Move to previous button or switch to volume section
            if controls.focused_button == 0 {
                controls.focused_section = Section::Volume;
            } else {
                controls.focused_button = (controls.focused_button - 1) % 8;
            }
        } else {
            // Move to next button or switch to volume section
            controls.focused_button = (controls.focused_button + 1) % 8;
            if controls.focused_button == 0 {
                controls.focused_section = Section::Volume;
            }
        }
    } else {
        // Switch back to controls section
        controls.focused_section = Section::Controls;
        controls.focused_button = if backward { 7 } else { 0 };
    }
}

fn handle_enter_action(controls: &Controls) -> Option<Action> {
    match controls.focused_section {
        Section::Controls => match controls.focused_button {
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

fn handle_mouse_event(controls: &mut Controls, event: MouseEvent) -> Option<Action> {
    match event {
        MouseEvent::Click { x, y } => {
            // First get and validate the area
            let area = match *controls.area.borrow() {
                Some(area) if is_point_in_rect(x, y, area) => area,
                _ => return None,
            };

            // Calculate section and button information
            let controls_width = (area.width * 8) / 10; // 80% of total width
            let relative_x = x - area.x;

            // Drop the area borrow before proceeding with state changes
            drop(controls.area.borrow());

            if relative_x <= controls_width {
                // Click in controls section
                controls.focused_section = Section::Controls;
                
                // Calculate button width and determine which button was clicked
                let button_width = controls_width / 8;
                let clicked_button = (relative_x / button_width) as usize;
                if clicked_button < 8 {
                    controls.focused_button = clicked_button;
                    // Instead of calling handle_key_event directly, we reconstruct the event
                    handle_event(controls, Event::Key(KeyEvent::Enter))
                } else {
                    None
                }
            } else {
                // Click in volume section
                controls.focused_section = Section::Volume;
                // TODO: Implement volume control click handling
                None
            }
        }
        MouseEvent::Scroll { .. } => None,
    }
}

fn is_point_in_rect(x: u16, y: u16, rect: ratatui::prelude::Rect) -> bool {
    x >= rect.x && x < rect.x + rect.width &&
    y >= rect.y && y < rect.y + rect.height
}
