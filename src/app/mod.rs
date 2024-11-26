use crate::components::{
    Component, LibraryBrowser, TrackList, TrackDetails,
    CurrentTrackInfo, PlaybackStatus, Controls, VolumeControl
};
use crate::events::{Event, EventResult, KeyEvent, FocusDirection};
use crate::state::AppState;
use crate::theme::Theme;
use crate::logger::Logger;
use anyhow::Result;

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
    pub library_browser: LibraryBrowser,
    pub track_list: TrackList,
    pub track_details: TrackDetails,
    // Secondary Row Components
    pub current_track_info: CurrentTrackInfo,
    pub playback_status: PlaybackStatus,
    // Control Row Components
    pub controls: Controls,
    pub volume_control: VolumeControl,
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
        
        // Initialize components
        let library_browser = LibraryBrowser::new();
        let track_list = TrackList::new();
        let track_details = TrackDetails::new();
        let current_track_info = CurrentTrackInfo::new();
        let playback_status = PlaybackStatus::new();
        let controls = Controls::new();
        let volume_control = VolumeControl::new();

        // Initialize managers
        let component_manager = ComponentManager::new();
        let event_manager = EventManager::new();
        let focus_manager = FocusManager::new();
        let logger = Logger::new()?;

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

        // Register components with both managers
        app.component_manager.register_components(
            &app.library_browser,
            &app.track_list,
            &app.track_details,
            &app.current_track_info,
            &app.playback_status,
            &app.controls,
            &app.volume_control,
        );

        app.event_manager.register_components(
            &app.library_browser,
            &app.track_list,
            &app.track_details,
            &app.current_track_info,
            &app.playback_status,
            &app.controls,
            &app.volume_control,
        );

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
        // Log the event, ignoring any logging errors to not disrupt the app
        let _ = self.logger.log_event(&event);

        // Handle focus-related events first
        match &event {
            Event::Key(KeyEvent::Tab) => {
                self.focus_manager.handle_event(&Event::Key(KeyEvent::Focus(FocusDirection::Next)))?;
            },
            Event::Key(KeyEvent::BackTab) => {
                self.focus_manager.handle_event(&Event::Key(KeyEvent::Focus(FocusDirection::Previous)))?;
            },
            _ => {
                self.focus_manager.handle_event(&event)?;
            }
        }
        
        // Process event through event manager
        self.event_manager.handle_event(event)?;
        
        // Update focus states after event processing
        self.update_focus_states();
        
        Ok(())
    }

    /// Updates focus states for all components and syncs with UI state
    pub fn update_focus_states(&mut self) {
        // Update the UI state to match FocusManager's current focus
        self.state.ui.focused_component = self.focus_manager.current_focus().to_string();

        // Update component focus states
        self.focus_manager.update_focus_states(
            &mut self.library_browser,
            &mut self.track_list,
            &mut self.track_details,
            &mut self.current_track_info,
            &mut self.playback_status,
            &mut self.controls,
            &mut self.volume_control,
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
    // TODO: Add integration tests
    // - Test component initialization
    // - Test event flow
    // - Test focus management
    // - Test theme loading
}
