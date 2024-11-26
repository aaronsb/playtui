use crate::events::{
    Event, Action, EventHandler, EventResult, SystemEvent, KeyEvent,
    PlayerAction, UIAction, AppAction
};
use crate::components::{
    Component, LibraryBrowser, TrackList, TrackDetails,
    CurrentTrackInfo, PlaybackStatus, Controls, VolumeControl
};
use crate::logger::Logger;
use std::rc::Rc;
use std::cell::RefCell;

/// Manages component registration and interaction
pub struct ComponentManager {
    components: Vec<Box<dyn EventHandler>>,
    logger: Logger,
}

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

impl ComponentManager {
    /// Creates a new ComponentManager
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
            logger: Logger::new().expect("Failed to initialize logger"),
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
        let _ = self.logger.log_debug("Components registered with ComponentManager");
    }

    /// Convert an action to an event
    fn action_to_event(action: &Action) -> Option<Event> {
        let event = match action {
            Action::Key(key_event) => Event::Key(key_event.clone()),
            Action::NavigateLeft => Event::Key(KeyEvent::Left),
            Action::NavigateRight => Event::Key(KeyEvent::Right),
            Action::NavigateUp => Event::Key(KeyEvent::Up),
            Action::NavigateDown => Event::Key(KeyEvent::Down),
            Action::Play => Event::Key(KeyEvent::Play),
            Action::Pause => Event::Key(KeyEvent::Pause),
            Action::Stop => Event::Key(KeyEvent::Stop),
            Action::NextTrack => Event::Key(KeyEvent::Next),
            Action::PreviousTrack => Event::Key(KeyEvent::Previous),
            Action::VolumeUp => Event::Key(KeyEvent::VolumeUp),
            Action::VolumeDown => Event::Key(KeyEvent::VolumeDown),
            Action::Player(player_action) => match player_action {
                PlayerAction::Play => Event::Key(KeyEvent::Play),
                PlayerAction::Pause => Event::Key(KeyEvent::Pause),
                PlayerAction::Stop => Event::Key(KeyEvent::Stop),
                PlayerAction::Record => Event::Key(KeyEvent::Record),
                PlayerAction::FastForward => Event::Key(KeyEvent::FastForward),
                PlayerAction::Rewind => Event::Key(KeyEvent::Rewind),
                PlayerAction::StopEject => Event::Key(KeyEvent::Stop),
                PlayerAction::SetVolume(_) | PlayerAction::LoadTrack(_) => Event::System(SystemEvent::TrackLoaded),
            },
            Action::UI(ui_action) => match ui_action {
                UIAction::Focus(direction) => Event::Key(KeyEvent::Focus(*direction)),
                UIAction::UpdateTheme(_) | UIAction::Resize { .. } => Event::System(SystemEvent::TrackLoaded),
            },
            Action::Playlist(_) | Action::Metadata(_) => Event::System(SystemEvent::TrackLoaded),
            Action::App(app_action) => match *app_action {
                AppAction::Error(ref msg) => Event::System(SystemEvent::Error(msg.clone())),
                AppAction::Quit => Event::Key(KeyEvent::Escape),
                AppAction::Cancel => Event::Key(KeyEvent::Escape),
                AppAction::NoOp => return None,
            },
            Action::Select => Event::Key(KeyEvent::Enter),
            Action::Back => Event::Key(KeyEvent::Escape),
            Action::Refresh => return None,
            Action::SetVolume(_) => Event::System(SystemEvent::TrackLoaded),
        };
        Some(event)
    }

    /// Updates all components with an action
    pub fn update_components(&mut self, action: Action) {
        let _ = self.logger.log_debug("\n=== Processing Action ===");
        let _ = self.logger.log_debug(&format!("Action: {:?}", action));
        
        let mut pending_actions = vec![action];
        let mut processed_actions = Vec::new();

        while let Some(current_action) = pending_actions.pop() {
            // Skip if we've already processed this action type
            if processed_actions.contains(&current_action) {
                let _ = self.logger.log_debug(&format!("Skipping already processed action: {:?}", current_action));
                continue;
            }
            processed_actions.push(current_action.clone());

            // Convert action to event
            if let Some(event) = Self::action_to_event(&current_action) {
                let _ = self.logger.log_debug(&format!("Converting action to event: {:?}", event));
                let _ = self.logger.log_event(&event);
                
                // Process the event through each component
                for component in &mut self.components {
                    if let Ok(Some(follow_up)) = component.handle_event(&event) {
                        let _ = self.logger.log_debug(&format!("Component generated follow-up action: {:?}", follow_up));
                        // Only add non-refresh follow-up actions that we haven't processed
                        if !matches!(follow_up, Action::Refresh) && !processed_actions.contains(&follow_up) {
                            pending_actions.push(follow_up);
                        }
                    }
                }
            } else {
                let _ = self.logger.log_debug(&format!("No event generated for action: {:?}", current_action));
            }
        }
        
        let _ = self.logger.log_debug("=== Action Processing Complete ===\n");
    }
}
