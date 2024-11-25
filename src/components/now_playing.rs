use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use super::Component;

#[derive(Default)]
pub struct NowPlaying {
    // Will add more fields later for playback state
}

impl Component for NowPlaying {
    fn new() -> Self {
        NowPlaying::default()
    }

    fn render(&self, frame: &mut Frame, area: Rect, focused: bool) {
        let block = Block::default()
            .title("Now Playing")
            .borders(Borders::ALL)
            .border_style(if focused {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            });

        frame.render_widget(
            Paragraph::new("Now Playing View").block(block),
            area,
        );
    }
}
