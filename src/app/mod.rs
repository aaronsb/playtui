mod event_dispatch;
mod event_processing;
mod initialization;
mod lifecycle;
mod state;
mod areas;
mod focus;

pub use event_dispatch::EventManager;

use ratatui::prelude::*;
use anyhow::Result;
use crate::events::{Event, Action, EventResult};
use crate::logger::Logger;
use crate::components::{
    Component, LibraryBrowser, TrackList, TrackDetails,
    CurrentTrackInfo, PlaybackStatus, Controls, VolumeControl
};
use crate::theme::Theme;
use crate::state::AppState;
use areas::AreaManager;
use focus::FocusManager;

use std::rc::Rc;
use std::cell::RefCell;

/// Component registry trait for managing component registration
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

/// Component manager for handling component updates and interactions
pub struct ComponentManager {
    components: Vec<Rc<RefCell<dyn Component>>>,
}

impl ComponentManager {
    pub fn new() -> Self {
        Self {
            components: Vec::new(),
        }
    }

    pub fn update_components(&mut self, action: Action) {
        let mut actions_to_process = vec![action];
        let mut processed_actions = Vec::new();

        while let Some(current_action) = actions_to_process.pop() {
            // Skip if we've already processed this action
            if processed_actions.contains(&current_action) {
                continue;
            }
            processed_actions.push(current_action.clone());

            // Update each component with the current action
            for component in &self.components {
                if let Ok(mut comp) = component.try_borrow_mut() {
                    if let Some(new_action) = comp.update(current_action.clone()) {
                        actions_to_process.push(new_action);
                    }
                }
            }
        }
    }
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
        self.components.clear();
        self.components.push(Rc::clone(library_browser) as Rc<RefCell<dyn Component>>);
        self.components.push(Rc::clone(track_list) as Rc<RefCell<dyn Component>>);
        self.components.push(Rc::clone(track_details) as Rc<RefCell<dyn Component>>);
        self.components.push(Rc::clone(current_track_info) as Rc<RefCell<dyn Component>>);
        self.components.push(Rc::clone(playback_status) as Rc<RefCell<dyn Component>>);
        self.components.push(Rc::clone(controls) as Rc<RefCell<dyn Component>>);
        self.components.push(Rc::clone(volume_control) as Rc<RefCell<dyn Component>>);
    }
}

/// Main application struct that ties all components together
pub struct App {
    pub state: AppState,
    pub theme: Theme,
    pub logger: Logger,
    pub event_manager: EventManager,
    pub component_manager: ComponentManager,
    pub focus_manager: FocusManager,
    pub area_manager: AreaManager,

    // UI Components
    pub library_browser: Rc<RefCell<LibraryBrowser>>,
    pub track_list: Rc<RefCell<TrackList>>,
    pub track_details: Rc<RefCell<TrackDetails>>,
    pub current_track_info: Rc<RefCell<CurrentTrackInfo>>,
    pub playback_status: Rc<RefCell<PlaybackStatus>>,
    pub controls: Rc<RefCell<Controls>>,
    pub volume_control: Rc<RefCell<VolumeControl>>,
}

impl App {
    /// Creates a new application instance
    pub fn new() -> Result<Self> {
        initialization::new()
    }

    /// Updates component areas in the UI
    pub fn update_component_area(&mut self, name: &str, area: Rect) {
        self.area_manager.update_area(name, area);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Add tests here
}
