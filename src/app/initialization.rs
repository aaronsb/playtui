use crate::components::{
    Component, LibraryBrowser, TrackList, TrackDetails,
    CurrentTrackInfo, PlaybackStatus, Controls, VolumeControl
};
use crate::state::AppState;
use crate::theme::Theme;
use crate::logger::Logger;
use anyhow::Result;
use std::rc::Rc;
use std::cell::RefCell;
use super::{App, ComponentManager, EventManager, FocusManager, AreaManager, ComponentRegistry};

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
    let area_manager = AreaManager::new();
    let logger = Logger::new()?;

    // Register components with both managers using cloned Rc references
    ComponentRegistry::register_components(
        &mut component_manager,
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
        area_manager,
        logger,
    };

    // Initialize focus states
    app.update_focus_states();

    Ok(app)
}
