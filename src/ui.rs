use ratatui::{
    prelude::*,
    widgets::{Block, Clear},
};
use crate::app::App;
use crate::components::Component;

pub fn render(frame: &mut Frame, app: &mut App) {
    // Clear the frame first
    frame.render_widget(Clear, frame.size());

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

    // Split Control Row (15%) into controls (80%) and volume (20%)
    let control_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(80),  // Controls
            Constraint::Percentage(20),  // Volume Control
        ])
        .split(main_chunks[2]);

    // Create a background block with the theme's background color
    let background = Block::default()
        .style(Style::default().bg(
            app.theme.get_color("background").unwrap_or(Color::Reset)
        ));
    frame.render_widget(background, frame.size());

    // Store areas and render Primary Row components
    app.update_component_area("library_browser", primary_chunks[0]);
    app.library_browser.borrow().render(
        frame,
        primary_chunks[0],
        app.state.ui.focused_component == "library_browser",
        &app.theme
    );

    app.update_component_area("track_list", primary_chunks[1]);
    app.track_list.borrow().render(
        frame,
        primary_chunks[1],
        app.state.ui.focused_component == "track_list",
        &app.theme
    );

    app.update_component_area("track_details", primary_chunks[2]);
    app.track_details.borrow().render(
        frame,
        primary_chunks[2],
        app.state.ui.focused_component == "track_details",
        &app.theme
    );

    // Store areas and render Secondary Row components
    app.update_component_area("current_track_info", secondary_chunks[0]);
    app.current_track_info.borrow().render(
        frame,
        secondary_chunks[0],
        app.state.ui.focused_component == "current_track_info",
        &app.theme
    );

    app.update_component_area("playback_status", secondary_chunks[1]);
    app.playback_status.borrow().render(
        frame,
        secondary_chunks[1],
        app.state.ui.focused_component == "playback_status",
        &app.theme
    );

    // Store areas and render Control Row components
    app.update_component_area("controls", control_chunks[0]);
    app.controls.borrow().render(
        frame,
        control_chunks[0],
        app.state.ui.focused_component == "controls",
        &app.theme
    );

    app.update_component_area("volume_control", control_chunks[1]);
    app.volume_control.borrow().render(
        frame,
        control_chunks[1],
        app.state.ui.focused_component == "volume_control",
        &app.theme
    );
}
