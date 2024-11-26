use crate::components::{
    Component, LibraryBrowser, TrackList, TrackDetails,
    CurrentTrackInfo, PlaybackStatus, Controls, VolumeControl
};
use crate::events::{Event, KeyEvent, FocusDirection, EventResult};

/// Manages focus state and navigation between components
pub struct FocusManager {
    component_order: Vec<String>,
    current_focus: usize,
}

impl FocusManager {
    /// Creates a new FocusManager with default component order
    pub fn new() -> Self {
        Self {
            component_order: vec![
                "library_browser".to_string(),
                "track_list".to_string(),
                "track_details".to_string(),
                "current_track_info".to_string(),
                "playback_status".to_string(),
                "controls".to_string(),
                "volume_control".to_string(),
            ],
            current_focus: 0,
        }
    }

    /// Returns the currently focused component name
    pub fn current_focus(&self) -> &str {
        &self.component_order[self.current_focus]
    }

    /// Moves focus in the specified direction
    pub fn move_focus(&mut self, direction: FocusDirection) {
        match direction {
            FocusDirection::Next => {
                self.current_focus = (self.current_focus + 1) % self.component_order.len();
            },
            FocusDirection::Previous => {
                if self.current_focus == 0 {
                    self.current_focus = self.component_order.len() - 1;
                } else {
                    self.current_focus -= 1;
                }
            },
        }
    }

    /// Updates component focus states based on current focus
    pub fn update_focus_states(
        &self,
        library_browser: &mut LibraryBrowser,
        track_list: &mut TrackList,
        track_details: &mut TrackDetails,
        current_track_info: &mut CurrentTrackInfo,
        playback_status: &mut PlaybackStatus,
        controls: &mut Controls,
        volume_control: &mut VolumeControl,
    ) {
        let focused = self.current_focus();
        Component::set_focused(library_browser, focused == "library_browser");
        Component::set_focused(track_list, focused == "track_list");
        Component::set_focused(track_details, focused == "track_details");
        Component::set_focused(current_track_info, focused == "current_track_info");
        Component::set_focused(playback_status, focused == "playback_status");
        Component::set_focused(controls, focused == "controls");
        Component::set_focused(volume_control, focused == "volume_control");
    }

    /// Handles focus-related events
    pub fn handle_event(&mut self, event: &Event) -> EventResult<()> {
        if let Event::Key(KeyEvent::Focus(direction)) = event {
            self.move_focus(*direction);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    // TODO: Add tests for focus management
    // - Test focus navigation
    // - Test component focus state updates
    // - Test focus event handling
    // - Test edge cases (first/last component)
}
