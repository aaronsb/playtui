// Primary Row Components
mod library_browser;
mod track_list;
mod track_details;

// Secondary Row Components
mod current_track_info;
mod playback_status;

// Control Row Components
mod prev_track;
mod play_pause;
mod next_track;
mod volume_control;

// Filesystem Module
pub mod filesystem;

// Re-export all components
pub use self::library_browser::LibraryBrowser;
pub use self::track_list::TrackList;
pub use self::track_details::TrackDetails;
pub use self::current_track_info::CurrentTrackInfo;
pub use self::playback_status::PlaybackStatus;
pub use self::prev_track::PrevTrack;
pub use self::play_pause::PlayPause;
pub use self::next_track::NextTrack;
pub use self::volume_control::VolumeControl;

use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};

// Re-export everything needed by components
pub use crate::events::{Event, Action, KeyEvent};

#[derive(Clone, Default)]
pub struct ComponentState {
    pub focused: bool,
}

pub trait Component: Clone + Send + 'static {
    fn new() -> Self where Self: Sized;
    fn render(&self, frame: &mut Frame, area: Rect, focused: bool);
    fn update(&mut self, action: Action) -> Option<Action>;
    fn focused(&self) -> bool;
    fn set_focused(&mut self, focused: bool);
    fn handle_event(&mut self, event: Event) -> Option<Action>;
}

// Helper function to create a styled block based on focus state
pub fn create_block<'a>(title: &'a str, focused: bool) -> Block<'a> {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(if focused {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default()
        })
}
