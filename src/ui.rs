use ratatui::{
    prelude::*,
    widgets::Block,
};
use crate::app::App;
use crate::components::Component;

pub fn render(frame: &mut Frame, app: &App) {
    // Create main vertical layout with three rows
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(60),  // Primary Row
            Constraint::Percentage(25),  // Secondary Row
            Constraint::Percentage(15),  // Control Row
        ])
        .split(frame.size());

    // Split Primary Row (60%) into three columns
    let primary_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(33),  // Library Browser
            Constraint::Percentage(34),  // Track List
            Constraint::Percentage(33),  // Track Details
        ])
        .split(main_chunks[0]);

    // Split Secondary Row (25%) into two columns
    let secondary_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),  // Current Track Info
            Constraint::Percentage(50),  // Playback Status
        ])
        .split(main_chunks[1]);

    // Split Control Row (15%) into four equal columns
    let control_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(25),  // Previous Track
            Constraint::Percentage(25),  // Play/Pause
            Constraint::Percentage(25),  // Next Track
            Constraint::Percentage(25),  // Volume Control
        ])
        .split(main_chunks[2]);

    // Create a background block with the theme's background color
    let background = Block::default()
        .style(Style::default().bg(
            app.theme.get_color("background").unwrap_or(Color::Reset)
        ));
    frame.render_widget(background, frame.size());

    // Render Primary Row components
    app.library_browser.render(
        frame,
        primary_chunks[0],
        app.state.ui.focused_component == "library_browser",
        &app.theme
    );
    app.track_list.render(
        frame,
        primary_chunks[1],
        app.state.ui.focused_component == "track_list",
        &app.theme
    );
    app.track_details.render(
        frame,
        primary_chunks[2],
        app.state.ui.focused_component == "track_details",
        &app.theme
    );

    // Render Secondary Row components
    app.current_track_info.render(
        frame,
        secondary_chunks[0],
        app.state.ui.focused_component == "current_track_info",
        &app.theme
    );
    app.playback_status.render(
        frame,
        secondary_chunks[1],
        app.state.ui.focused_component == "playback_status",
        &app.theme
    );

    // Render Control Row components
    app.prev_track.render(
        frame,
        control_chunks[0],
        app.state.ui.focused_component == "prev_track",
        &app.theme
    );
    app.play_pause.render(
        frame,
        control_chunks[1],
        app.state.ui.focused_component == "play_pause",
        &app.theme
    );
    app.next_track.render(
        frame,
        control_chunks[2],
        app.state.ui.focused_component == "next_track",
        &app.theme
    );
    app.volume_control.render(
        frame,
        control_chunks[3],
        app.state.ui.focused_component == "volume_control",
        &app.theme
    );
}
