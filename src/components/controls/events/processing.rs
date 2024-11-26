use crate::events::{Event, Action};
use crate::components::Component;
use super::super::Controls;

pub(crate) fn preprocess_event(_event: &Event) -> bool {
    // For now, we just pass all events through
    // In the future, we could add event filtering or transformation logic here
    true
}

pub(crate) fn postprocess_action(controls: &Controls, action: Option<Action>) -> Option<Action> {
    // For now, we just pass actions through
    // In the future, we could add action validation or transformation logic here
    // For example, we might want to prevent certain actions based on the control's state
    match action {
        Some(Action::Player(_)) if !controls.focused() => None,
        _ => action
    }
}
