use crate::events::{Action, KeyEvent, MouseEvent, NavigationEvent, PlayerAction};
use crate::components::Component;
use super::super::{Controls, Section};
use super::types::get_action_for_button;
use ratatui::prelude::Rect;

pub(crate) fn handle_key_event(controls: &mut Controls, key_event: KeyEvent) -> Option<Action> {
    match key_event {
        KeyEvent::Left => {
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
        KeyEvent::Right => {
            if controls.focused_section == Section::Controls {
                // Move to next button
                controls.focused_button = (controls.focused_button + 1) % 8;
                Some(Action::Refresh)
            } else {
                None
            }
        },
        KeyEvent::Tab => {
            handle_tab_navigation(controls, false);
            Some(Action::Refresh)
        },
        KeyEvent::BackTab => {
            handle_tab_navigation(controls, true);
            Some(Action::Refresh)
        },
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

pub(crate) fn handle_navigation(controls: &mut Controls, event: NavigationEvent) -> Option<Action> {
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

pub(crate) fn handle_mouse_event(controls: &mut Controls, event: MouseEvent) -> Option<Action> {
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
                    handle_enter_action(controls)
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
    get_action_for_button(controls.focused_section, controls.focused_button)
}

fn is_point_in_rect(x: u16, y: u16, rect: Rect) -> bool {
    x >= rect.x && x < rect.x + rect.width &&
    y >= rect.y && y < rect.y + rect.height
}
