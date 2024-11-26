mod playlist;
mod now_playing;
mod controls;

pub use self::playlist::Playlist;
pub use self::now_playing::NowPlaying;
pub use self::controls::Controls;

use ratatui::{
    prelude::*,
    widgets::{Block, Borders},
};

// Re-export everything needed by components
pub use crate::events::{Event, Action};

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
