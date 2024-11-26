// Primary Row Components
mod library_browser;
mod track_list;
mod track_details;

// Secondary Row Components
mod current_track_info;
mod playback_status;

// Control Row Components
mod controls;
mod volume_control;

// Filesystem Module
pub mod filesystem;

// Re-export all components
pub use self::library_browser::LibraryBrowser;
pub use self::track_list::TrackList;
pub use self::track_details::TrackDetails;
pub use self::current_track_info::CurrentTrackInfo;
pub use self::playback_status::PlaybackStatus;
pub use self::controls::Controls;
pub use self::volume_control::VolumeControl;

use ratatui::{
    prelude::*,
    widgets::{Block, Borders, BorderType},
};

// Re-export everything needed by components
pub use crate::events::{Event, Action, KeyEvent};
pub use crate::theme::Theme;

#[derive(Clone, Default)]
pub struct ComponentState {
    pub focused: bool,
}

pub trait Component: Clone + Send + 'static {
    fn new() -> Self where Self: Sized;
    fn render(&self, frame: &mut Frame, area: Rect, focused: bool, theme: &Theme);
    fn update(&mut self, action: Action) -> Option<Action>;
    fn focused(&self) -> bool;
    fn set_focused(&mut self, focused: bool);
    fn handle_event(&mut self, event: Event) -> Option<Action>;
}

// Helper function to create a styled block based on focus state and theme
pub fn create_block<'a>(title: &'a str, focused: bool, theme: &Theme) -> Block<'a> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(if focused { BorderType::Thick } else { BorderType::Rounded })
        .border_style(
            if focused {
                theme.get_style("border_focused")
            } else {
                theme.get_style("border_unfocused")
            }
        )
}
