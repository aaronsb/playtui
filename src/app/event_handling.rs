use crate::events::{Event, Action, EventResult, EventError, SystemEvent, KeyEvent, UIAction, AppAction, FocusDirection, NavigationEvent};
use crate::components::{
    LibraryBrowser, TrackList, TrackDetails,
    CurrentTrackInfo, PlaybackStatus, Controls, VolumeControl
};
use super::components::ComponentManager;
use std::rc::Rc;
use std::cell::RefCell;

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
    pub fn orient_and_decide(&self, event: Event) -> EventResult<Action> {
        match event {
            Event::System(SystemEvent::Error(e)) => {
                Err(EventError::DispatchError(format!("Error event received: {}", e)))
            },
            Event::Key(key_event) => {
                match key_event {
                    // Tab navigation between frames
                    KeyEvent::Tab => Ok(Action::UI(UIAction::Focus(FocusDirection::Next))),
                    KeyEvent::BackTab => Ok(Action::UI(UIAction::Focus(FocusDirection::Previous))),
                    KeyEvent::Focus(direction) => Ok(Action::UI(UIAction::Focus(direction))),
                    
                    // Convert arrow keys to navigation events
                    KeyEvent::Left => Ok(Action::NavigateLeft),
                    KeyEvent::Right => Ok(Action::NavigateRight),
                    KeyEvent::Up => Ok(Action::NavigateUp),
                    KeyEvent::Down => Ok(Action::NavigateDown),
                    
                    // Pass through other key events directly
                    _ => Ok(Action::Key(key_event)),
                }
            },
            Event::Navigation(nav_event) => {
                match nav_event {
                    NavigationEvent::Left => Ok(Action::NavigateLeft),
                    NavigationEvent::Right => Ok(Action::NavigateRight),
                    NavigationEvent::Up => Ok(Action::NavigateUp),
                    NavigationEvent::Down => Ok(Action::NavigateDown),
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
        library_browser: &Rc<RefCell<LibraryBrowser>>,
        track_list: &Rc<RefCell<TrackList>>,
        track_details: &Rc<RefCell<TrackDetails>>,
        current_track_info: &Rc<RefCell<CurrentTrackInfo>>,
        playback_status: &Rc<RefCell<PlaybackStatus>>,
        controls: &Rc<RefCell<Controls>>,
        volume_control: &Rc<RefCell<VolumeControl>>,
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
