use crate::app::{App, PlaybackState, Focus, MenuPage};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect, Alignment},
    style::Style,
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Clear, Gauge},
    Frame,
};

fn format_time(seconds: u64) -> String {
    let minutes = seconds / 60;
    let seconds = seconds % 60;
    format!("{:02}:{:02}", minutes, seconds)
}

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

    // Draw menu if it's visible
    if app.show_menu {
        draw_menu(frame, app);
    }
}

fn draw_menu(frame: &mut Frame, app: &App) {
    let area = frame.size();
    
    // Calculate menu size (80% of screen)
    let menu_width = (area.width as f32 * 0.8) as u16;
    let menu_height = (area.height as f32 * 0.8) as u16;
    let menu_x = (area.width - menu_width) / 2;
    let menu_y = (area.height - menu_height) / 2;

    let menu_area = Rect::new(menu_x, menu_y, menu_width, menu_height);

    // Draw a clear widget first to create a blank canvas
    frame.render_widget(Clear, menu_area);

    // Get menu content based on current page
    let content = match app.menu_page {
        MenuPage::Preferences => "Preferences Menu\n\nPlaceholder for preferences settings",
        MenuPage::Looks => "Looks Menu\n\nPlaceholder for appearance settings",
        MenuPage::About => "About Menu\n\nPlaceholder for app information",
    };

    // Get menu title based on current page
    let title = match app.menu_page {
        MenuPage::Preferences => " Preferences ",
        MenuPage::Looks => " Looks ",
        MenuPage::About => " About ",
    };

    // Draw the menu
    let menu = Paragraph::new(format!("{}\n\nPress Tab to switch pages\nPress 'm' or 'q' to close", content))
        .block(Block::default()
            .borders(Borders::ALL)
            .border_style(app.theme.menu_border_style())
            .title(title)
            .title_alignment(Alignment::Center))
        .alignment(Alignment::Center)
        .style(app.theme.menu_style());

    frame.render_widget(menu, menu_area);
}

fn draw_now_playing(frame: &mut Frame, app: &App, area: Rect) {
    // Split the now playing area into left (track info) and right (progress)
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Left side - track info (dynamically sized)
            Constraint::Length(20),     // Right side - progress (fixed width)
        ])
        .split(area);

    // Left side - Track info
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
            .border_style(app.theme.now_playing_border_style()))
        .style(app.theme.now_playing_style());

    frame.render_widget(now_playing, chunks[0]);

    // Right side - Progress
    if let Some(track) = &app.current_track {
        if let Some(duration) = track.duration {
            let played = format_time(app.playback_position);
            let total = format_time(duration);
            let remaining = format_time(duration.saturating_sub(app.playback_position));
            
            let ratio = app.playback_position as f64 / duration as f64;
            let percentage = ratio * 100.0;
            
            let title = format!(" {} / {} / {} ", played, total, remaining);
            let label = format!("{:.1}%", percentage);

            let gauge = Gauge::default()
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title(title)
                    .border_style(app.theme.progress_gauge_border_style()))
                .gauge_style(app.theme.progress_gauge_style())
                .ratio(ratio)
                .label(label)
                .use_unicode(true);

            frame.render_widget(gauge, chunks[1]);
        } else {
            let progress = Paragraph::new("--:--/--:--/--:--")
                .block(Block::default()
                    .borders(Borders::ALL)
                    .title(" Progress ")
                    .border_style(app.theme.progress_text_border_style()))
                .alignment(Alignment::Center)
                .style(app.theme.progress_text_style());

            frame.render_widget(progress, chunks[1]);
        }
    } else {
        let progress = Paragraph::new("--:--/--:--/--:--")
            .block(Block::default()
                .borders(Borders::ALL)
                .title(" Progress ")
                .border_style(app.theme.progress_text_border_style()))
            .alignment(Alignment::Center)
            .style(app.theme.progress_text_style());

        frame.render_widget(progress, chunks[1]);
    }
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

    let current_dir = app.filesystem.get_current_dir_display();
    let title = format!(" Browser [{}] ", current_dir);

    let browser = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(title)
            .border_style(app.theme.browser_border_style(app.focus == Focus::Browser)))
        .highlight_style(app.theme.browser_highlight_style())
        .highlight_symbol(app.theme.browser_highlight_symbol());

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
        .map(|(_i, track)| {
            let title = track.title.as_deref().unwrap_or("Unknown");
            let artist = track.artist.as_deref().unwrap_or("Unknown Artist");
            let content = format!("{} - {}", format_title(title), artist);
            
            let style = if app.focus == Focus::Songs && app.current_track.as_ref() == Some(track) && app.playback_state != PlaybackState::Stopped {
                app.theme.songs_playing_style()
            } else {
                Style::default()
            };

            ListItem::new(Line::from(vec![
                Span::styled(content, style)
            ]))
        })
        .collect();

    let songs = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" Songs ")
            .border_style(app.theme.songs_border_style(app.focus == Focus::Songs)))
        .highlight_style(app.theme.songs_highlight_style())
        .highlight_symbol(app.theme.songs_highlight_symbol());

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
        .map(|(_i, track)| {
            let title = track.title.as_deref().unwrap_or("Unknown");
            let artist = track.artist.as_deref().unwrap_or("Unknown Artist");
            let content = format!("{} - {}", format_title(title), artist);
            
            let style = if app.focus == Focus::Playlist && app.current_track.as_ref() == Some(track) && app.playback_state != PlaybackState::Stopped {
                app.theme.playlist_playing_style()
            } else {
                Style::default()
            };

            ListItem::new(Line::from(vec![
                Span::styled(content, style)
            ]))
        })
        .collect();

    let playlist = List::new(items)
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" Playlist ")
            .border_style(app.theme.playlist_border_style(app.focus == Focus::Playlist)))
        .highlight_style(app.theme.playlist_highlight_style())
        .highlight_symbol(app.theme.playlist_highlight_symbol());

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
        Span::raw(".: Next | "),
        Span::raw(",: Previous | "),
        Span::raw("+/-: Volume | "),
        Span::raw("↑/↓: Navigate | "),
        Span::raw("←/→: Add/Remove | "),
        Span::raw("Tab: Switch Focus | "),
        Span::raw("m: Menu | "),
        Span::raw("q: Quit | "),
        Span::styled(volume, app.theme.controls_volume_style()),
    ];

    let controls_widget = Paragraph::new(Line::from(controls))
        .block(Block::default()
            .borders(Borders::ALL)
            .title(" Controls ")
            .border_style(app.theme.controls_border_style()))
        .style(app.theme.controls_style());

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
