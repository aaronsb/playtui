use ratatui::{
    prelude::*,
    widgets::{Block, Borders, BorderType},
};
use crate::events::{Event, Action};
use crate::theme::Theme;

pub mod current_track_info;
pub mod library_browser;
pub mod now_playing;
pub mod playback_status;
pub mod playlist;
pub mod track_details;
pub mod track_list;
pub mod volume_control;
pub mod controls;
pub mod filesystem;
#[cfg(test)]
mod tests;

pub use current_track_info::CurrentTrackInfo;
pub use library_browser::LibraryBrowser;
pub use now_playing::NowPlaying;
pub use playback_status::PlaybackStatus;
pub use playlist::*;
pub use track_details::TrackDetails;
pub use track_list::TrackList;
pub use volume_control::VolumeControl;
pub use controls::*;
pub use filesystem::*;

#[derive(Default, Clone)]
pub struct ComponentState {
    pub focused: bool,
}

pub trait Component {
    fn new() -> Self where Self: Sized;
    fn render(&self, frame: &mut Frame, area: Rect, focused: bool, theme: &Theme);
    fn update(&mut self, action: Action) -> Option<Action>;
    fn handle_event(&mut self, event: Event) -> Option<Action>;
    fn focused(&self) -> bool;
    fn set_focused(&mut self, focused: bool);
}

// The block is created with references that live as long as the input references
pub fn create_block<'a>(title: &'a str, focused: bool, theme: &'a Theme) -> Block<'a> {
    let border_style = if focused {
        theme.get_style("border_focused")
            .add_modifier(Modifier::BOLD)
    } else {
        theme.get_style("border_unfocused")
    };

    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(if focused { BorderType::Thick } else { BorderType::Rounded })
        .border_style(border_style)
}
