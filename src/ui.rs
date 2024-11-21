use crate::app::{App, PlaybackState, Focus};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame,
};

pub fn draw(frame: &mut Frame, app: &App) {
    // Create main vertical layout
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Now Playing
            Constraint::Min(0),     // Middle section (Browser + Songs + Playlist)
            Constraint::Length(3),  // Controls
        ])
        .margin(1)
        .split(frame.size());

    // Split middle section horizontally for Browser, Songs, and Playlist
    let middle_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // Browser
            Constraint::Percentage(40), // Songs
            Constraint::Percentage(40), // Playlist
        ])
        .split(main_chunks[1]);

    draw_now_playing(frame, app, main_chunks[0]);
    draw_browser(frame, app, middle_chunks[0]);
    draw_songs(frame, app, middle_chunks[1]);
    draw_playlist(frame, app, middle_chunks[2]);
    draw_controls(frame, app, main_chunks[2]);
}

fn draw_now_playing(frame: &mut Frame, app: &App, area: Rect) {
    let title = app.current_track
        .as_ref()
        .and_then(|t| t.title.as_ref())
        .map(|s| format_title(s))
        .unwrap_or_else(|| "No track playing".to_string());

    let artist = app.current_track
        .as_ref()
        .and_then(|t| t.artist.as_ref())
        .map(|s| format!(" - {}", s))
        .unwrap_or_default();

    let playback_symbol = match app.playback_state {
        PlaybackState::Playing => "▶",
        PlaybackState::Paused => "⏸",
        PlaybackState::Stopped => "⏹",
    };

    let text = format!("{} {} {}", playback_symbol, title, artist);
    
    let now_playing = Paragraph::new(text)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" Now Playing ")
            .border_style(Style::default().fg(Color::Cyan)))
        .style(Style::default().fg(Color::White));

    frame.render_widget(now_playing, area);
}

fn draw_browser(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = (0..app.filesystem.entries.len())
        .map(|i| {
            let name = app.filesystem.get_entry_name(i);
            ListItem::new(Line::from(vec![
                Span::styled(format!("📁 {}", name), Style::default())
            ]))
        })
        .collect();

    let border_style = if app.focus == Focus::Browser {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Cyan)
    };

    let current_dir = app.filesystem.get_current_dir_display();
    let title = format!(" Browser [{}] ", current_dir);

    let browser = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(border_style))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("► ");

    frame.render_stateful_widget(
        browser,
        area,
        &mut ratatui::widgets::ListState::default().with_selected(Some(app.filesystem.selected_entry)),
    );
}

fn draw_songs(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .songs
        .iter()
        .enumerate()
        .map(|(i, track)| {
            let title = track.title.as_deref().unwrap_or("Unknown");
            let artist = track.artist.as_deref().unwrap_or("Unknown Artist");
            let content = format!("{} - {}", format_title(title), artist);
            
            let style = if app.focus == Focus::Songs && app.current_track.as_ref() == Some(track) && app.playback_state != PlaybackState::Stopped {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            };

            ListItem::new(Line::from(vec![
                Span::styled(content, style)
            ]))
        })
        .collect();

    let border_style = if app.focus == Focus::Songs {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Cyan)
    };

    let songs = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" Songs ")
            .border_style(border_style))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("► ");

    frame.render_stateful_widget(
        songs,
        area,
        &mut ratatui::widgets::ListState::default().with_selected(Some(app.selected_song_index)),
    );
}

fn draw_playlist(frame: &mut Frame, app: &App, area: Rect) {
    let items: Vec<ListItem> = app
        .playlist
        .iter()
        .enumerate()
        .map(|(i, track)| {
            let title = track.title.as_deref().unwrap_or("Unknown");
            let artist = track.artist.as_deref().unwrap_or("Unknown Artist");
            let content = format!("{} - {}", format_title(title), artist);
            
            let style = if app.focus == Focus::Playlist && app.current_track.as_ref() == Some(track) && app.playback_state != PlaybackState::Stopped {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            };

            ListItem::new(Line::from(vec![
                Span::styled(content, style)
            ]))
        })
        .collect();

    let border_style = if app.focus == Focus::Playlist {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Cyan)
    };

    let playlist = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" Playlist ")
            .border_style(border_style))
        .highlight_style(
            Style::default()
                .bg(Color::DarkGray)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("► ");

    frame.render_stateful_widget(
        playlist,
        area,
        &mut ratatui::widgets::ListState::default().with_selected(Some(app.selected_playlist_index)),
    );
}

fn draw_controls(frame: &mut Frame, app: &App, area: Rect) {
    let volume = format!("Volume: {:3}%", app.volume);
    let controls = vec![
        Span::raw("Space: Play/Pause | "),
        Span::raw("n: Next | "),
        Span::raw("p: Previous | "),
        Span::raw("+/-: Volume | "),
        Span::raw("↑/↓: Navigate | "),
        Span::raw("←/→: Add/Remove | "),
        Span::raw("Tab: Switch Focus | "),
        Span::raw("q: Quit | "),
        Span::styled(volume, Style::default().fg(Color::Cyan)),
    ];

    let controls_widget = Paragraph::new(Line::from(controls))
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" Controls ")
            .border_style(Style::default().fg(Color::Cyan)))
        .style(Style::default().fg(Color::White));

    frame.render_widget(controls_widget, area);
}

fn format_title(title: &str) -> String {
    // Remove file extension if present
    let title = title.trim_end_matches(".flac")
        .trim_end_matches(".mp3")
        .trim_end_matches(".ogg")
        .trim_end_matches(".wav");
    
    // Remove any leading numbers and separators
    let title = if let Some(pos) = title.find(char::is_alphabetic) {
        &title[pos..]
    } else {
        title
    };

    // Clean up any remaining artifacts
    title.trim()
        .replace("  ", " ")
        .replace('_', " ")
        .replace('-', " - ")
        .trim()
        .to_string()
}
