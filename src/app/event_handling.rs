use crate::events::{Event, Action, EventResult, EventError, SystemEvent, KeyEvent, UIAction, AppAction, FocusDirection};
use crate::components::{
    LibraryBrowser, TrackList, TrackDetails,
    CurrentTrackInfo, PlaybackStatus, Controls, VolumeControl
};
use super::components::ComponentManager;

/// Manages event processing and dispatching
pub struct EventManager {
    component_manager: ComponentManager,
}

impl EventManager {
    /// Creates a new EventManager
    pub fn new() -> Self {
        Self {
            component_manager: ComponentManager::new(),
        }
    }

    /// Handles incoming events and manages their flow through the system
    pub fn handle_event(&mut self, event: Event) -> EventResult<()> {
        // First phase: Observe - collect and process the event
        self.observe_event(&event)?;

        // Second phase: Orient - analyze the event context
        let action = self.orient_and_decide(event)?;

        // Third phase: Act - execute the determined action
        self.act_on_action(action)
    }

    /// Observe phase: Initial event processing
    fn observe_event(&self, _event: &Event) -> EventResult<()> {
        // Log or monitor event if needed
        // In the future, this could include event validation, filtering, etc.
        Ok(())
    }

    /// Orient and Decide phases: Analyze event and determine appropriate action
    fn orient_and_decide(&self, event: Event) -> EventResult<Action> {
        match event {
            Event::System(SystemEvent::Error(e)) => {
                Err(EventError::DispatchError(format!("Error event received: {}", e)))
            },
            Event::Key(key_event) => {
                match key_event {
                    KeyEvent::Tab => Ok(Action::UI(UIAction::Focus(FocusDirection::Next))),
                    KeyEvent::BackTab => Ok(Action::UI(UIAction::Focus(FocusDirection::Previous))),
                    KeyEvent::Focus(direction) => Ok(Action::UI(UIAction::Focus(direction))),
                    KeyEvent::Esc => Ok(Action::App(AppAction::Quit)),
                    _ => Ok(Action::App(AppAction::NoOp)),
                }
            },
            _ => Ok(Action::App(AppAction::NoOp)),
        }
    }

    /// Act phase: Execute the determined action
    fn act_on_action(&mut self, action: Action) -> EventResult<()> {
        // Update components with the action
        self.component_manager.update_components(action);
        Ok(())
    }

    /// Register components with the event manager
    pub fn register_components(&mut self,
        library_browser: &LibraryBrowser,
        track_list: &TrackList,
        track_details: &TrackDetails,
        current_track_info: &CurrentTrackInfo,
        playback_status: &PlaybackStatus,
        controls: &Controls,
        volume_control: &VolumeControl,
    ) {
        self.component_manager.register_components(
            library_browser,
            track_list,
            track_details,
            current_track_info,
            playback_status,
            controls,
            volume_control,
        );
    }
}
