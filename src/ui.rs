use ratatui::prelude::*;
use crate::app::App;
use crate::components::Component;

pub fn render(frame: &mut Frame, app: &App) {
    // Create a layout with three vertical chunks
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(60),  // Playlist
            Constraint::Percentage(20),  // Now Playing
            Constraint::Percentage(20),  // Controls
        ])
        .split(frame.size());

    // Render components
    app.playlist.render(
        frame,
        chunks[0],
        app.state.ui.focused_component == "playlist"
    );
    app.now_playing.render(
        frame,
        chunks[1],
        app.state.ui.focused_component == "nowplaying"
    );
    app.controls.render(
        frame,
        chunks[2],
        app.state.ui.focused_component == "controls"
    );
}
