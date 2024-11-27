use std::rc::Rc;
use std::cell::RefCell;
use crate::events::{
    Event, EventResult, KeyEvent, FocusDirection, MouseEvent,
    EventError, Action, AppAction, EventDispatcher, EventHandler
};
use crate::components::{
    Component, LibraryBrowser, TrackList, TrackDetails,
    CurrentTrackInfo, PlaybackStatus, Controls, VolumeControl
};

use super::App;

/// Wrapper for components that implement the EventHandler trait
struct ComponentWrapper<T: Component> {
    component: Rc<RefCell<T>>,
}

impl<T: Component> EventHandler for ComponentWrapper<T> {
    fn handle_event(&mut self, event: &Event) -> EventResult<Option<Action>> {
        if let Ok(mut component) = self.component.try_borrow_mut() {
            Ok(component.handle_event(event.clone()))
        } else {
            Ok(None)
        }
    }

    fn can_handle(&self, _event: &Event) -> bool {
        true // Let handle_event determine if it can handle the event
    }
}

/// Manages event processing and routing for the application
pub struct EventManager {
    dispatcher: EventDispatcher,
    components: Vec<Box<dyn EventHandler>>,
}

impl EventManager {
    pub fn new() -> Self {
        Self {
            dispatcher: EventDispatcher::new(),
            components: Vec::new(),
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

    pub fn orient_and_decide(&mut self, event: Event) -> Result<Action, EventError> {
        // Use dispatcher to collect actions from handlers
        let actions = self.dispatcher.dispatch(&event)?;
        
        // Return first valid action or NoOp if none
        Ok(actions.into_iter().next().unwrap_or(Action::App(AppAction::NoOp)))
    }
}

impl App {
    /// Handles incoming events
    pub fn handle_event(&mut self, event: Event) -> EventResult<()> {
        // Log the incoming event and current state
        let _ = self.logger.log_debug("=== Event Processing Start ===");
        let _ = self.logger.log_event(&event);
        let _ = self.logger.log_debug(&format!("Current focus: {}", self.focus_manager.current_focus()));

        match &event {
            // Mouse Events - Check position and update focus before processing
            Event::Mouse(mouse_event) => {
                match mouse_event {
                    MouseEvent::Click { x, y } => {
                        // Check if click is in a component area
                        if let Some(component_name) = self.area_manager.component_at_position(*x, *y) {
                            // Update focus if clicking unfocused component
                            if component_name != self.focus_manager.current_focus() {
                                let _ = self.logger.log_debug(&format!("Focusing component from click: {}", component_name));
                                self.focus_manager.set_focus(component_name);
                                self.update_focus_states();
                            }
                            
                            // Now process the click event
                            if let Ok(action) = self.event_manager.orient_and_decide(event.clone()) {
                                let _ = self.logger.log_debug(&format!("Generated action from click: {:?}", action));
                                self.component_manager.update_components(action);
                            }
                        }
                    },
                    MouseEvent::Scroll { .. } => {
                        // Only process scroll events for the focused component
                        let _focused_component = self.focus_manager.current_focus();
                        if let Ok(action) = self.event_manager.orient_and_decide(event.clone()) {
                            let _ = self.logger.log_debug(&format!("Generated action from scroll: {:?}", action));
                            self.component_manager.update_components(action);
                        }
                    }
                }
            },

            // Global Navigation Events - Always process
            Event::Key(KeyEvent::Tab) => {
                let _ = self.logger.log_debug("Processing Tab event for global navigation");
                self.focus_manager.handle_event(&Event::Key(KeyEvent::Focus(FocusDirection::Next)))?;
                self.update_focus_states();
                let _ = self.logger.log_debug(&format!("New focus after Tab: {}", self.focus_manager.current_focus()));
            },
            Event::Key(KeyEvent::BackTab) => {
                let _ = self.logger.log_debug("Processing BackTab event for global navigation");
                self.focus_manager.handle_event(&Event::Key(KeyEvent::Focus(FocusDirection::Previous)))?;
                self.update_focus_states();
                let _ = self.logger.log_debug(&format!("New focus after BackTab: {}", self.focus_manager.current_focus()));
            },
            Event::Key(KeyEvent::Focus(direction)) => {
                let _ = self.logger.log_debug(&format!("Processing Focus event: {:?}", direction));
                self.focus_manager.handle_event(&Event::Key(KeyEvent::Focus(*direction)))?;
                self.update_focus_states();
                let _ = self.logger.log_debug(&format!("New focus after Focus event: {}", self.focus_manager.current_focus()));
            },

            // Frame-Specific Events - Only process if component is focused
            Event::Key(KeyEvent::Left | KeyEvent::Right | KeyEvent::Up | KeyEvent::Down | KeyEvent::Enter) => {
                let focused_component = self.focus_manager.current_focus();
                let _ = self.logger.log_debug(&format!("Processing frame-specific event for {}: {:?}", focused_component, event));
                
                if self.focus_manager.should_process_event(&event, focused_component) {
                    let _ = self.logger.log_debug("Component should process event");
                    if let Ok(action) = self.event_manager.orient_and_decide(event.clone()) {
                        let _ = self.logger.log_debug(&format!("Generated action: {:?}", action));
                        self.component_manager.update_components(action);
                    } else {
                        let _ = self.logger.log_debug("Failed to generate action from event");
                    }
                } else {
                    let _ = self.logger.log_debug("Event ignored - component not focused");
                }
            },

            // Global Hotkeys - Always process
            Event::Key(KeyEvent::Space | KeyEvent::Quit | KeyEvent::Escape |
                      KeyEvent::Play | KeyEvent::Pause | KeyEvent::Stop |
                      KeyEvent::Next | KeyEvent::Previous |
                      KeyEvent::VolumeUp | KeyEvent::VolumeDown) => {
                let _ = self.logger.log_debug("Processing global hotkey");
                if let Ok(action) = self.event_manager.orient_and_decide(event.clone()) {
                    let _ = self.logger.log_debug(&format!("Generated action from hotkey: {:?}", action));
                    self.component_manager.update_components(action);
                } else {
                    let _ = self.logger.log_debug("Failed to generate action from hotkey");
                }
            },

            // System Events - Always process
            Event::System(_) => {
                let _ = self.logger.log_debug("Processing system event");
                if let Ok(action) = self.event_manager.orient_and_decide(event.clone()) {
                    let _ = self.logger.log_debug(&format!("Generated action from system event: {:?}", action));
                    self.component_manager.update_components(action);
                } else {
                    let _ = self.logger.log_debug("Failed to generate action from system event");
                }
            },

            // Other Events - Process through focus manager first
            _ => {
                let _ = self.logger.log_debug("Processing other event type");
                self.focus_manager.handle_event(&event)?;
                self.update_focus_states();

                if let Ok(action) = self.event_manager.orient_and_decide(event.clone()) {
                    let _ = self.logger.log_debug(&format!("Generated action from other event: {:?}", action));
                    self.component_manager.update_components(action);
                } else {
                    let _ = self.logger.log_debug("Failed to generate action from other event");
                }
            },
        }

        let _ = self.logger.log_debug("=== Event Processing End ===");
        Ok(())
    }
}
