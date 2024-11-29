use std::rc::Rc;
use std::cell::RefCell;
use crate::events::{
    Event, EventResult, Action, AppAction,
    EventDispatcher, EventHandler, EventError
};
use crate::components::{
    Component, LibraryBrowser, TrackList, TrackDetails,
    CurrentTrackInfo, PlaybackStatus, Controls, VolumeControl
};

/// Wrapper for components that implement the EventHandler trait
struct ComponentWrapper<T: Component> {
    component: Rc<RefCell<T>>,
}

impl<T: Component> EventHandler for ComponentWrapper<T> {
    fn handle_event(&mut self, event: &Event) -> EventResult<Option<Action>> {
        if let Ok(mut component) = self.component.try_borrow_mut() {
            Ok(component.handle_event(*event))
        } else {
            Ok(None)
        }
    }

    fn can_handle(&self, _event: &Event) -> bool {
        true // Let handle_event determine if it can handle the event
    }
}

/// Manages event dispatch and routing for the application
pub struct EventManager {
    dispatcher: EventDispatcher,
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            dispatcher: EventDispatcher::new(),
        }
    }

    pub fn register_components(
        &mut self,
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

        // Register components with the dispatcher
        for component in components {
            self.dispatcher.register_handler(component);
        }
    }

    pub fn dispatch_event(&mut self, event: &Event) -> EventResult<Action> {
        // Use dispatcher to collect actions from handlers
        let actions = self.dispatcher.dispatch(event)
            .map_err(|e| EventError::DispatchError(e.to_string()))?;
        
        // Return first valid action or NoOp if none
        Ok(actions.into_iter().next().unwrap_or(Action::App(AppAction::NoOp)))
    }
}
