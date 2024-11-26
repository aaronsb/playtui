use crate::components::{
    Component, LibraryBrowser, TrackList, TrackDetails,
    CurrentTrackInfo, PlaybackStatus, Controls, VolumeControl
};
use crate::state::AppState;
use crate::theme::Theme;
use crate::logger::Logger;
use std::rc::Rc;
use std::cell::RefCell;
use ratatui::prelude::Rect;

use super::{ComponentManager, EventManager, FocusManager, AreaManager};

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
    pub(crate) component_manager: ComponentManager,
    pub(crate) event_manager: EventManager,
    pub(crate) focus_manager: FocusManager,
    pub(crate) area_manager: AreaManager,
    pub(crate) logger: Logger,
}

impl App {
    /// Updates the area for a component during rendering
    pub fn update_component_area(&mut self, component_name: &str, area: Rect) {
        self.area_manager.update_area(component_name, area);
    }
}
