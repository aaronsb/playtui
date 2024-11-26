use crate::components::{Component, ComponentState};
use crate::events::{Event, Action};

mod state;
mod events;
mod view;

#[cfg(test)]
mod tests;

use state::VolumeState;

pub struct VolumeControl {
    component_state: ComponentState,
    state: VolumeState,
}

impl Clone for VolumeControl {
    fn clone(&self) -> Self {
        Self {
            component_state: self.component_state.clone(),
            state: self.state.clone(),
        }
    }
}

impl Component for VolumeControl {
    fn new() -> Self {
        Self {
            component_state: ComponentState::default(),
            state: VolumeState::default(),
        }
    }

    fn render(&self, frame: &mut ratatui::prelude::Frame, area: ratatui::prelude::Rect, focused: bool, theme: &crate::theme::Theme) {
        view::render(&self.state, frame, area, focused, theme);
    }

    fn update(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::VolumeUp => {
                let volume = self.state.increase_volume();
                Some(Action::SetVolume(volume))
            }
            Action::VolumeDown => {
                let volume = self.state.decrease_volume();
                Some(Action::SetVolume(volume))
            }
            Action::SetVolume(vol) => {
                self.state.set_volume(vol);
                None
            }
            _ => None,
        }
    }

    fn focused(&self) -> bool {
        self.component_state.focused
    }

    fn set_focused(&mut self, focused: bool) {
        self.component_state.focused = focused;
    }

    fn handle_event(&mut self, event: Event) -> Option<Action> {
        // Get focused state before borrowing state mutably
        let is_focused = self.focused();
        events::handle_event(&mut self.state, event, is_focused)
    }
}
