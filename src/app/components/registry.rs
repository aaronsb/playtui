use std::rc::Rc;
use std::cell::RefCell;
use crate::events::{
    Event, Action, EventHandler, EventResult, KeyEvent // Import KeyEvent directly
};
use crate::components::{
    Component, LibraryBrowser, TrackList, TrackDetails,
    CurrentTrackInfo, PlaybackStatus, Controls, VolumeControl
};

use super::ComponentManager;

/// Wrapper for components that implement the EventHandler trait
struct ComponentWrapper<T: Component> {
    component: Rc<RefCell<T>>,
}

impl<T: Component + 'static> EventHandler for ComponentWrapper<T> {
    fn handle_event(&mut self, event: &Event) -> EventResult<Option<Action>> {
        match event {
            // System events are always handled
            Event::System(_) => Ok(self.component.borrow_mut().handle_event(event.clone())),
            
            // Key events need special handling
            Event::Key(key_event) => {
                let requires_focus = match key_event {
                    // Navigation keys require focus
                    KeyEvent::Tab | KeyEvent::BackTab |
                    KeyEvent::Left | KeyEvent::Right |
                    KeyEvent::Up | KeyEvent::Down |
                    KeyEvent::Enter | KeyEvent::Focus(_) => true,
                    
                    // Playback control keys don't require focus
                    KeyEvent::Space | // Pause/Play
                    KeyEvent::Play | KeyEvent::Pause | KeyEvent::Stop |
                    KeyEvent::Next | KeyEvent::Previous |
                    KeyEvent::VolumeUp | KeyEvent::VolumeDown |
                    KeyEvent::Record | KeyEvent::FastForward |
                    KeyEvent::Rewind => false,
                    
                    // Other keys require focus
                    _ => true,
                };

                if !requires_focus || self.component.borrow().focused() {
                    Ok(self.component.borrow_mut().handle_event(event.clone()))
                } else {
                    Ok(None)
                }
            },
            
            // Mouse events require focus
            Event::Mouse(_) => {
                if self.component.borrow().focused() {
                    Ok(self.component.borrow_mut().handle_event(event.clone()))
                } else {
                    Ok(None)
                }
            },
            
            // Navigation events require focus
            Event::Navigation(_) => {
                if self.component.borrow().focused() {
                    Ok(self.component.borrow_mut().handle_event(event.clone()))
                } else {
                    Ok(None)
                }
            },
        }
    }

    fn can_handle(&self, _event: &Event) -> bool {
        // Let handle_event determine if it can handle the event
        true
    }
}

/// Trait for registering components with the ComponentManager
pub trait ComponentRegistry {
    fn register_components(
        &mut self,
        library_browser: &Rc<RefCell<LibraryBrowser>>,
        track_list: &Rc<RefCell<TrackList>>,
        track_details: &Rc<RefCell<TrackDetails>>,
        current_track_info: &Rc<RefCell<CurrentTrackInfo>>,
        playback_status: &Rc<RefCell<PlaybackStatus>>,
        controls: &Rc<RefCell<Controls>>,
        volume_control: &Rc<RefCell<VolumeControl>>,
    );
}

impl ComponentRegistry for ComponentManager {
    fn register_components(
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

        self.components = components;
        let _ = self.logger.log_debug("Components registered with ComponentManager");
    }
}
