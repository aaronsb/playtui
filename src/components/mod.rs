use ratatui::{
    prelude::*,
    widgets::{Block, Borders, BorderType},
};
use crate::events::{Event, Action};
use crate::theme::Theme;

mod current_track_info;
mod library_browser;
mod now_playing;
mod playback_status;
mod playlist;
mod track_details;
mod track_list;
mod volume_control;
mod controls;
mod filesystem;

pub use current_track_info::*;
pub use library_browser::*;
pub use now_playing::*;
pub use playback_status::*;
pub use playlist::*;
pub use track_details::*;
pub use track_list::*;
pub use volume_control::*;
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
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_type(if focused { BorderType::Thick } else { BorderType::Rounded })
        .border_style(if focused {
            theme.get_style("border_focused")
        } else {
            theme.get_style("border_unfocused")
        })
}
