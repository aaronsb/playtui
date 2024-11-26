use crate::components::{
    Component, LibraryBrowser, TrackList, TrackDetails,
    CurrentTrackInfo, PlaybackStatus, Controls, VolumeControl
};
use crate::events::{Event, EventResult};
use crate::state::AppState;
use crate::theme::Theme;
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
        let mut component_manager = ComponentManager::new();
        component_manager.register_components(
            &library_browser,
            &track_list,
            &track_details,
            &current_track_info,
            &playback_status,
            &controls,
            &volume_control,
        );

        let event_manager = EventManager::new(component_manager);
        let focus_manager = FocusManager::new();

        Ok(App {
            state: AppState::default(),
            theme,
            library_browser,
            track_list,
            track_details,
            current_track_info,
            playback_status,
            controls,
            volume_control,
            component_manager: ComponentManager::new(), // This will be updated in register_components
            event_manager,
            focus_manager,
        })
    }

    /// Loads a theme from the specified path
    pub fn load_theme(&mut self, path: &str) -> Result<()> {
        self.theme = Theme::load(path)?;
        Ok(())
    }

    /// Handles incoming events
    pub fn handle_event(&mut self, event: Event) -> EventResult<()> {
        // Handle focus-related events
        self.focus_manager.handle_event(&event)?;
        
        // Update focus states
        self.update_focus_states();

        // Process event through event manager
        self.event_manager.handle_event(event)
    }

    /// Updates focus states for all components
    pub fn update_focus_states(&mut self) {
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
        self.handle_event(Event::Key(crate::events::KeyEvent::Focus(
            crate::events::FocusDirection::Next
        )))
    }

    /// Moves focus to the previous component
    pub fn previous_frame(&mut self) -> EventResult<()> {
        self.handle_event(Event::Key(crate::events::KeyEvent::Focus(
            crate::events::FocusDirection::Previous
        )))
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
