use crate::components::{
    Component, LibraryBrowser, TrackList, TrackDetails,
    CurrentTrackInfo, PlaybackStatus, Controls, VolumeControl
};
use crate::events::{Event, Action, EventHandler, EventResult, SystemEvent};

/// Manages component registration and interaction
pub struct ComponentManager {
    components: Vec<Box<dyn EventHandler>>,
}

/// Wrapper for components that implement the EventHandler trait
struct ComponentWrapper<T: Component> {
    component: T,
}

impl<T: Component> EventHandler for ComponentWrapper<T> {
    fn handle_event(&mut self, event: &Event) -> EventResult<Option<Action>> {
        Ok(self.component.handle_event(event.clone()))
    }

    fn can_handle(&self, _event: &Event) -> bool {
        self.component.focused()
    }
}

impl ComponentManager {
    /// Creates a new ComponentManager
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    /// Registers all application components
    pub fn register_components(&mut self,
        library_browser: &LibraryBrowser,
        track_list: &TrackList,
        track_details: &TrackDetails,
        current_track_info: &CurrentTrackInfo,
        playback_status: &PlaybackStatus,
        controls: &Controls,
        volume_control: &VolumeControl,
    ) {
        // Register each component with a wrapper
        let components: Vec<Box<dyn EventHandler>> = vec![
            Box::new(ComponentWrapper { component: library_browser.clone() }),
            Box::new(ComponentWrapper { component: track_list.clone() }),
            Box::new(ComponentWrapper { component: track_details.clone() }),
            Box::new(ComponentWrapper { component: current_track_info.clone() }),
            Box::new(ComponentWrapper { component: playback_status.clone() }),
            Box::new(ComponentWrapper { component: controls.clone() }),
            Box::new(ComponentWrapper { component: volume_control.clone() }),
        ];

        self.components = components;
    }

    /// Updates all components with an action
    pub fn update_components(&mut self, initial_action: Action) {
        let mut pending_actions = vec![initial_action];
        let state_event = Event::System(SystemEvent::TrackLoaded);

        // Process actions until no new ones are generated
        while !pending_actions.is_empty() {
            let current_actions = std::mem::take(&mut pending_actions);
            
            // Process each action
            for action in current_actions {
                // Collect new actions from each component
                for component in self.components.iter_mut() {
                    if let Ok(Some(follow_up)) = component.handle_event(&state_event) {
                        pending_actions.push(follow_up);
                    }
                }
            }
        }
    }
}
