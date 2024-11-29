use crate::events::{Event, EventResult, KeyEvent, FocusDirection};

use super::App;

impl App {
    /// Updates focus states for all components and syncs with UI state
    pub fn update_focus_states(&mut self) {
        // Update the UI state to match FocusManager's current focus
        self.state.ui.focused_component = self.focus_manager.current_focus().to_string();

        // Update component focus states
        self.focus_manager.update_focus_states(
            &mut self.library_browser.borrow_mut(),
            &mut self.track_list.borrow_mut(),
            &mut self.track_details.borrow_mut(),
            &mut self.current_track_info.borrow_mut(),
            &mut self.playback_status.borrow_mut(),
            &mut self.controls.borrow_mut(),
            &mut self.volume_control.borrow_mut(),
        );
    }

    /// Moves focus to the next component
    pub fn next_frame(&mut self) -> EventResult<()> {
        // Use handle_event to process the focus change
        self.handle_event(Event::Key(KeyEvent::Focus(FocusDirection::Next)))
    }

    /// Moves focus to the previous component
    pub fn previous_frame(&mut self) -> EventResult<()> {
        // Use handle_event to process the focus change
        self.handle_event(Event::Key(KeyEvent::Focus(FocusDirection::Previous)))
    }
}
