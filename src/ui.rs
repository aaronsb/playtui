use ratatui::prelude::*;

use crate::app::{App, FocusedFrame};
use crate::components::Component;

pub fn render(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(25),
            Constraint::Percentage(15),
        ])
        .split(frame.size());

    // Render playlist
    app.playlist.render(frame, chunks[0], matches!(app.focused_frame, FocusedFrame::Playlist));

    // Render now playing
    app.now_playing.render(frame, chunks[1], matches!(app.focused_frame, FocusedFrame::NowPlaying));

    // Render controls
    app.controls.render(frame, chunks[2], matches!(app.focused_frame, FocusedFrame::Controls));
}
