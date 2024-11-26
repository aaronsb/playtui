use crate::events::{Event, Action, EventHandler, EventResult, SystemEvent, KeyEvent, NavigationEvent};
use crate::components::{
    Component, LibraryBrowser, TrackList, TrackDetails,
    CurrentTrackInfo, PlaybackStatus, Controls, VolumeControl
};
use std::rc::Rc;
use std::cell::RefCell;

/// Manages component registration and interaction
pub struct ComponentManager {
    components: Vec<Box<dyn EventHandler>>,
}

/// Wrapper for components that implement the EventHandler trait
struct ComponentWrapper<T: Component> {
    component: Rc<RefCell<T>>,
}

impl<T: Component + 'static> EventHandler for ComponentWrapper<T> {
    fn handle_event(&mut self, event: &Event) -> EventResult<Option<Action>> {
        Ok(self.component.borrow_mut().handle_event(event.clone()))
    }

    fn can_handle(&self, event: &Event) -> bool {
        match event {
            // Navigation events can be handled regardless of focus
            Event::Navigation(_) => true,
            // For key events that generate navigation actions, also allow regardless of focus
            Event::Key(KeyEvent::Left | KeyEvent::Right | KeyEvent::Up | KeyEvent::Down) => true,
            // All other events require focus
            _ => self.component.borrow().focused()
        }
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
        library_browser: &Rc<RefCell<LibraryBrowser>>,
        track_list: &Rc<RefCell<TrackList>>,
        track_details: &Rc<RefCell<TrackDetails>>,
        current_track_info: &Rc<RefCell<CurrentTrackInfo>>,
        playback_status: &Rc<RefCell<PlaybackStatus>>,
        controls: &Rc<RefCell<Controls>>,
        volume_control: &Rc<RefCell<VolumeControl>>,
    ) {
        // Register each component with a wrapper
        let components: Vec<Box<dyn EventHandler>> = vec![
            Box::new(ComponentWrapper { component: Rc::clone(library_browser) }),
            Box::new(ComponentWrapper { component: Rc::clone(track_list) }),
            Box::new(ComponentWrapper { component: Rc::clone(track_details) }),
            Box::new(ComponentWrapper { component: Rc::clone(current_track_info) }),
            Box::new(ComponentWrapper { component: Rc::clone(playback_status) }),
            Box::new(ComponentWrapper { component: Rc::clone(controls) }),
            Box::new(ComponentWrapper { component: Rc::clone(volume_control) }),
        ];

        self.components = components;
    }

    /// Updates all components with an action
    pub fn update_components(&mut self, action: Action) {
        let mut pending_actions = vec![action];

        // Process actions until no new ones are generated
        while !pending_actions.is_empty() {
            let current_actions = std::mem::take(&mut pending_actions);
            
            // Process each action
            for action in current_actions {
                // Convert navigation actions to events
                let event = match &action {
                    Action::NavigateLeft => Event::Navigation(NavigationEvent::Left),
                    Action::NavigateRight => Event::Navigation(NavigationEvent::Right),
                    Action::NavigateUp => Event::Navigation(NavigationEvent::Up),
                    Action::NavigateDown => Event::Navigation(NavigationEvent::Down),
                    Action::Key(key_event) => Event::Key(key_event.clone()),
                    _ => Event::System(SystemEvent::TrackLoaded),
                };

                // Process the event through all components
                for component in self.components.iter_mut() {
                    if component.can_handle(&event) {
                        if let Ok(Some(follow_up)) = component.handle_event(&event) {
                            pending_actions.push(follow_up);
                        }
                    }
                }
            }
        }
    }
}
