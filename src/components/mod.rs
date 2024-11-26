use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders};
use crate::events::{Event, Action};
use crate::theme::Theme;

pub mod current_track_info;
pub mod now_playing;
pub mod playback_status;
pub mod controls;
pub mod library_browser;
pub mod track_list;
pub mod track_details;
pub mod volume_control;
pub mod playlist;
pub mod filesystem;

#[cfg(test)]
pub mod tests;

pub use current_track_info::CurrentTrackInfo;
pub use now_playing::NowPlaying;
pub use playback_status::PlaybackStatus;
pub use controls::Controls;
pub use library_browser::LibraryBrowser;
pub use track_list::TrackList;
pub use track_details::TrackDetails;
pub use volume_control::VolumeControl;
pub use playlist::Playlist;

#[derive(Clone, Debug, PartialEq)]
pub struct ComponentState {
    pub focused: bool,
}

impl Default for ComponentState {
    fn default() -> Self {
        Self {
            focused: false,
        }
    }
}

pub trait Component {
    fn new() -> Self where Self: Sized;
    fn render(&self, frame: &mut Frame, area: Rect, focused: bool, theme: &Theme);
    fn update(&mut self, action: Action) -> Option<Action>;
    fn handle_event(&mut self, event: Event) -> Option<Action>;
    fn focused(&self) -> bool;
    fn set_focused(&mut self, focused: bool);
}

pub fn create_block<'a>(title: &'a str, focused: bool, theme: &Theme) -> Block<'a> {
    let border_style = if focused {
        Style::default()
            .fg(theme.styles.border_focused.fg.as_ref().map(|c| parse_color(c)).unwrap_or(Color::White))
            .add_modifier(theme.styles.border_focused.modifiers.as_ref()
                .map(|m| parse_modifiers(m))
                .unwrap_or(Modifier::empty()))
    } else {
        Style::default()
            .fg(theme.styles.border_unfocused.fg.as_ref().map(|c| parse_color(c)).unwrap_or(Color::Gray))
            .add_modifier(theme.styles.border_unfocused.modifiers.as_ref()
                .map(|m| parse_modifiers(m))
                .unwrap_or(Modifier::empty()))
    };

    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(border_style)
}

fn parse_color(color_str: &str) -> Color {
    // Simple hex color parsing
    if color_str.starts_with('#') && color_str.len() == 7 {
        if let (Ok(r), Ok(g), Ok(b)) = (
            u8::from_str_radix(&color_str[1..3], 16),
            u8::from_str_radix(&color_str[3..5], 16),
            u8::from_str_radix(&color_str[5..7], 16),
        ) {
            return Color::Rgb(r, g, b);
        }
    }
    Color::White // fallback
}

fn parse_modifiers(modifiers: &[String]) -> Modifier {
    let mut result = Modifier::empty();
    for modifier in modifiers {
        match modifier.as_str() {
            "BOLD" => result |= Modifier::BOLD,
            "DIM" => result |= Modifier::DIM,
            "ITALIC" => result |= Modifier::ITALIC,
            "UNDERLINED" => result |= Modifier::UNDERLINED,
            "SLOW_BLINK" => result |= Modifier::SLOW_BLINK,
            "RAPID_BLINK" => result |= Modifier::RAPID_BLINK,
            "REVERSED" => result |= Modifier::REVERSED,
            "HIDDEN" => result |= Modifier::HIDDEN,
            "CROSSED_OUT" => result |= Modifier::CROSSED_OUT,
            _ => {}
        }
    }
    result
}
