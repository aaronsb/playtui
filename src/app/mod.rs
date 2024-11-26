use crate::components::{
    Component, LibraryBrowser, TrackList, TrackDetails,
    CurrentTrackInfo, PlaybackStatus, Controls, VolumeControl
};
use crate::events::{Event, EventResult, KeyEvent, FocusDirection};
use crate::state::AppState;
use crate::theme::Theme;
use crate::logger::Logger;
use anyhow::Result;
use std::rc::Rc;
use std::cell::RefCell;

// Declare submodules
mod components;
mod event_handling;
mod focus;

// Re-export public items
pub use components::ComponentManager;
pub use event_handling::EventManager;
pub use focus::FocusManager;

/// Main application struct that coordinates all components and manages the application state
pub struct App {
    pub state: AppState,
    pub theme: Theme,
    // Primary Row Components
    pub library_browser: Rc<RefCell<LibraryBrowser>>,
    pub track_list: Rc<RefCell<TrackList>>,
    pub track_details: Rc<RefCell<TrackDetails>>,
    // Secondary Row Components
    pub current_track_info: Rc<RefCell<CurrentTrackInfo>>,
    pub playback_status: Rc<RefCell<PlaybackStatus>>,
    // Control Row Components
    pub controls: Rc<RefCell<Controls>>,
    pub volume_control: Rc<RefCell<VolumeControl>>,
    // Managers
    component_manager: ComponentManager,
    event_manager: EventManager,
    focus_manager: FocusManager,
    logger: Logger,
}

impl App {
    /// Creates a new App instance with default configuration
    pub fn new() -> Result<App> {
        let theme = Theme::load_default()?;
        
        // Initialize components wrapped in Rc<RefCell>
        let library_browser = Rc::new(RefCell::new(LibraryBrowser::new()));
        let track_list = Rc::new(RefCell::new(TrackList::new()));
        let track_details = Rc::new(RefCell::new(TrackDetails::new()));
        let current_track_info = Rc::new(RefCell::new(CurrentTrackInfo::new()));
        let playback_status = Rc::new(RefCell::new(PlaybackStatus::new()));
        let controls = Rc::new(RefCell::new(Controls::new()));
        let volume_control = Rc::new(RefCell::new(VolumeControl::new()));

        // Initialize managers
        let mut component_manager = ComponentManager::new();
        let mut event_manager = EventManager::new();
        let focus_manager = FocusManager::new();
        let logger = Logger::new()?;

        // Register components with both managers using cloned Rc references
        component_manager.register_components(
            &library_browser,
            &track_list,
            &track_details,
            &current_track_info,
            &playback_status,
            &controls,
            &volume_control,
        );

        event_manager.register_components(
            &library_browser,
            &track_list,
            &track_details,
            &current_track_info,
            &playback_status,
            &controls,
            &volume_control,
        );

        // Create App instance
        let mut app = App {
            state: AppState::default(),
            theme,
            library_browser,
            track_list,
            track_details,
            current_track_info,
            playback_status,
            controls,
            volume_control,
            component_manager,
            event_manager,
            focus_manager,
            logger,
        };

        // Initialize focus states
        app.update_focus_states();

        Ok(app)
    }

    /// Loads a theme from the specified path
    pub fn load_theme(&mut self, path: &str) -> Result<()> {
        self.theme = Theme::load(path)?;
        Ok(())
    }

    /// Handles incoming events
    pub fn handle_event(&mut self, event: Event) -> EventResult<()> {
        // Log the incoming event and current state
        let _ = self.logger.log_debug("=== Event Processing Start ===");
        let _ = self.logger.log_event(&event);
        let _ = self.logger.log_debug(&format!("Current focus: {}", self.focus_manager.current_focus()));

        match &event {
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
        self.handle_event(Event::Key(KeyEvent::Focus(FocusDirection::Next)))
    }

    /// Moves focus to the previous component
    pub fn previous_frame(&mut self) -> EventResult<()> {
        self.handle_event(Event::Key(KeyEvent::Focus(FocusDirection::Previous)))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_global_navigation_events() {
        let mut app = App::new().unwrap();
        let initial_focus = app.focus_manager.current_focus().to_string();
        
        // Tab should always change focus
        app.handle_event(Event::Key(KeyEvent::Tab)).unwrap();
        assert_ne!(app.focus_manager.current_focus(), initial_focus);
    }

    #[test]
    fn test_frame_specific_events() {
        let mut app = App::new().unwrap();
        let focused_component = app.focus_manager.current_focus().to_string();
        
        // Arrow keys should only work on focused component
        app.handle_event(Event::Key(KeyEvent::Left)).unwrap();
        assert_eq!(app.focus_manager.current_focus(), focused_component);
    }

    #[test]
    fn test_global_hotkeys() {
        let mut app = App::new().unwrap();
        let initial_focus = app.focus_manager.current_focus().to_string();
        
        // Space should work regardless of focus
        app.handle_event(Event::Key(KeyEvent::Space)).unwrap();
        assert_eq!(app.focus_manager.current_focus(), initial_focus);
    }
}
