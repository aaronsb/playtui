use crate::events::{Event, Action};
use crate::components::Component;
use super::Controls;

mod handlers;
mod types;
mod processing;

pub fn handle_event(controls: &mut Controls, event: Event) -> Option<Action> {
    // Pre-process the event
    if !processing::preprocess_event(&event) {
        return None;
    }

    let action = match event {
        Event::Key(key_event) => {
            // All key events require focus now
            if !controls.focused() {
                return None;
            }
            handlers::handle_key_event(controls, key_event)
        },
        Event::Mouse(mouse_event) => {
            if !controls.focused() {
                return None;
            }
            handlers::handle_mouse_event(controls, mouse_event)
        },
        Event::Navigation(nav_event) => handlers::handle_navigation(controls, nav_event),
        Event::System(_) => None,
    };

    // Post-process the action
    processing::postprocess_action(controls, action)
}
