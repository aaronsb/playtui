mod playlist;
mod now_playing;
mod controls;

pub use self::playlist::Playlist;
pub use self::now_playing::NowPlaying;
pub use self::controls::Controls;

use ratatui::prelude::*;

pub trait Component {
    fn new() -> Self where Self: Sized;
    fn render(&self, frame: &mut Frame, area: Rect, focused: bool);
}
