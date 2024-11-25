use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use super::Component;

#[derive(Default)]
pub struct Playlist {
    // Will add more fields later for actual playlist functionality
}

impl Component for Playlist {
    fn new() -> Self {
        Playlist::default()
    }

    fn render(&self, frame: &mut Frame, area: Rect, focused: bool) {
        let block = Block::default()
            .title("Playlist")
            .borders(Borders::ALL)
            .border_style(if focused {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            });

        frame.render_widget(
            Paragraph::new("Playlist View").block(block),
            area,
        );
    }
}
