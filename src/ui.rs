use ratatui::{
    prelude::*,
    widgets::*,
};
use crate::app::{App, FocusedPane};

pub fn draw(f: &mut Frame, app: &App) {
    // Main layout: 2 vertical sections (no more controls bar)
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(20),     // Main content area
            Constraint::Length(1),   // Status bar only
        ])
        .split(f.size());

    // Split main content area into 3 columns
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(30),  // File browser
            Constraint::Percentage(40),  // Now playing
            Constraint::Percentage(30),  // Playlist
        ])
        .split(main_chunks[0]);

    // Draw main content
    draw_file_browser(f, app, content_chunks[0]);
    draw_now_playing(f, app, content_chunks[1]);
    draw_playlist(f, app, content_chunks[2]);
    
    // Draw status bar
    draw_status_bar(f, app, main_chunks[1]);
}

fn draw_file_browser(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app.file_list.entries
        .iter()
        .map(|path| {
            let name = if path.to_str() == Some("..") {
                "..".to_string()
            } else {
                path.file_name()
                    .unwrap_or_default()
                    .to_str()
                    .unwrap_or_default()
                    .to_string()
            };
            
            let prefix = if path.is_dir() { "📁 " } else if is_audio_file(path) { "🎵 " } else { "   " };
            ListItem::new(format!("{}{}", prefix, name))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title("Library")
                .border_style(get_border_style(app.focused_pane == FocusedPane::Files))
        )
        .highlight_style(Style::default().bg(Color::DarkGray))
        .highlight_symbol("▶ ");

    let mut state = ListState::default();
    state.select(Some(app.file_list.selected));
    
    f.render_stateful_widget(list, area, &mut state);
}

fn draw_now_playing(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(10),  // Cassette
            Constraint::Length(3),   // Progress bar
            Constraint::Length(3),   // Track info
            Constraint::Length(3),   // Volume
            Constraint::Min(1),      // Empty space
        ])
        .split(area);

    // Draw cassette
    draw_cassette(f, app, chunks[0]);
    
    // Draw progress bar with time
    draw_progress(f, app, chunks[1]);
    
    // Draw track info
    draw_track_info(f, app, chunks[2]);
    
    // Draw volume
    draw_volume(f, app, chunks[3]);
}

fn draw_cassette(f: &mut Frame, app: &App, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title("Now Playing");
    
    let inner_area = block.inner(area);
    f.render_widget(block, area);
    
    // Get track names for cassette
    let mut tracks = vec!["".to_string(), "".to_string(), "".to_string()];
    
    if let Some(current_idx) = app.playlist.current_track {
        if let Some(track) = app.playlist.tracks.get(current_idx) {
            tracks[0] = track.file_stem()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .to_string();
        }
        
        if let Some(track) = app.playlist.tracks.get(current_idx + 1) {
            tracks[1] = track.file_stem()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default()
                .to_string();
        }
    }
    
    let cassette = create_compact_cassette(&tracks, app.player_state.is_playing);
    let paragraph = Paragraph::new(cassette)
        .alignment(Alignment::Center);
    
    f.render_widget(paragraph, inner_area);
}

fn create_compact_cassette(tracks: &[String], is_playing: bool) -> Vec<Line<'static>> {
    let format_track = |track: &str, max_len: usize| -> String {
        if track.is_empty() {
            " ".repeat(max_len)
        } else if track.len() > max_len {
            format!("{}...", &track[..max_len-3])
        } else {
            format!("{:^width$}", track, width = max_len)
        }
    };
    
    let track1 = format_track(&tracks[0], 20);
    let track2 = format_track(tracks.get(1).unwrap_or(&"".to_string()), 20);
    
    let reel = if is_playing { "◉" } else { "◯" };
    let status = if is_playing { "▶ Playing" } else { "⏸ Paused" };
    
    vec![
        Line::from("┌──────────────────────────┐"),
        Line::from(format!("│ A  {}  │", track1)),
        Line::from(format!("│    {}  │", track2)),
        Line::from(format!("│  {}  ████████████  {}  │", reel, reel)),
        Line::from("│      ░░░░░░░░░░      │"),
        Line::from("└──────────────────────────┘"),
        Line::from(format!("        {}", status)),
    ]
}

fn draw_progress(f: &mut Frame, app: &App, area: Rect) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(8),   // Time elapsed
            Constraint::Min(10),     // Progress bar
            Constraint::Length(8),   // Time total
        ])
        .split(area);

    // Format duration as MM:SS
    fn format_duration(duration: std::time::Duration) -> String {
        let total_secs = duration.as_secs();
        let mins = total_secs / 60;
        let secs = total_secs % 60;
        format!("{:02}:{:02}", mins, secs)
    }

    // Time elapsed
    let elapsed = format_duration(app.player_state.current_position);
    let elapsed_widget = Paragraph::new(elapsed)
        .block(Block::default().borders(Borders::TOP | Borders::BOTTOM | Borders::LEFT))
        .alignment(Alignment::Center);
    f.render_widget(elapsed_widget, chunks[0]);

    // Progress bar
    let progress = if app.player_state.total_duration.as_secs() > 0 {
        let current = app.player_state.current_position.as_secs() as f64;
        let total = app.player_state.total_duration.as_secs() as f64;
        ((current / total) * 100.0) as u16
    } else {
        0
    };
    
    let progress_bar = Gauge::default()
        .block(Block::default().borders(Borders::TOP | Borders::BOTTOM))
        .gauge_style(Style::default().fg(Color::Cyan))
        .percent(progress.min(100));
    f.render_widget(progress_bar, chunks[1]);

    // Total time
    let total = format_duration(app.player_state.total_duration);
    let total_widget = Paragraph::new(total)
        .block(Block::default().borders(Borders::TOP | Borders::BOTTOM | Borders::RIGHT))
        .alignment(Alignment::Center);
    f.render_widget(total_widget, chunks[2]);
}

fn draw_track_info(f: &mut Frame, app: &App, area: Rect) {
    let info = if let Some(track_idx) = app.playlist.current_track {
        if let Some(track) = app.playlist.tracks.get(track_idx) {
            let name = track.file_stem()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default();
            format!("🎵 {}", name)
        } else {
            "No track selected".to_string()
        }
    } else {
        "No track selected".to_string()
    };

    let paragraph = Paragraph::new(info)
        .block(Block::default().borders(Borders::ALL))
        .alignment(Alignment::Center);

    f.render_widget(paragraph, area);
}

fn draw_playlist(f: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app.playlist.tracks
        .iter()
        .enumerate()
        .map(|(i, path)| {
            let name = path.file_stem()
                .unwrap_or_default()
                .to_str()
                .unwrap_or_default();
            
            let prefix = if Some(i) == app.playlist.current_track {
                if app.player_state.is_playing { "▶ " } else { "⏸ " }
            } else {
                "  "
            };
            
            ListItem::new(format!("{}{}", prefix, name))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(format!("Playlist ({} tracks)", app.playlist.tracks.len()))
                .border_style(get_border_style(app.focused_pane == FocusedPane::Playlist))
        )
        .highlight_style(Style::default().bg(Color::DarkGray))
        .highlight_symbol("▶ ");

    let mut state = ListState::default();
    state.select(Some(app.playlist.selected));
    
    f.render_stateful_widget(list, area, &mut state);
}


fn draw_volume(f: &mut Frame, app: &App, area: Rect) {
    let volume_bar = Gauge::default()
        .block(Block::default().borders(Borders::ALL).title("Volume"))
        .gauge_style(Style::default().fg(Color::Green))
        .percent(app.player_state.volume as u16)
        .label(format!("{}%", app.player_state.volume));

    f.render_widget(volume_bar, area);
}

fn draw_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let help = match app.focused_pane {
        FocusedPane::Files => "Enter: Add to playlist | Tab: Switch | +/-: Volume | q: Quit",
        FocusedPane::Playlist => "Enter: Play | d: Remove | c: Clear | Tab: Switch | q: Quit",
        _ => "SPC: Play/Pause | Tab: Switch | +/-: Volume | ←/→: Prev/Next | q: Quit",
    };
    
    let paragraph = Paragraph::new(help)
        .style(Style::default().fg(Color::DarkGray));
    
    f.render_widget(paragraph, area);
}

fn get_border_style(focused: bool) -> Style {
    if focused {
        Style::default().fg(Color::Cyan)
    } else {
        Style::default().fg(Color::DarkGray)
    }
}

fn is_audio_file(path: &std::path::PathBuf) -> bool {
    if let Some(ext) = path.extension() {
        matches!(
            ext.to_str().unwrap_or("").to_lowercase().as_str(),
            "mp3" | "flac" | "ogg" | "wav" | "m4a" | "aac"
        )
    } else {
        false
    }
}