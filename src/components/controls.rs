use ratatui::{
    prelude::*,
    widgets::{Block, Borders, Paragraph},
};

use super::Component;

#[derive(Default)]
pub struct Controls {
    // Will add more fields later for control state
}

impl Component for Controls {
    fn new() -> Self {
        Controls::default()
    }

    fn render(&self, frame: &mut Frame, area: Rect, focused: bool) {
        let block = Block::default()
            .title("Controls")
            .borders(Borders::ALL)
            .border_style(if focused {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default()
            });

        frame.render_widget(
            Paragraph::new("Controls View").block(block),
            area,
        );
    }
}
