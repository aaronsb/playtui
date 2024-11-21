use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use playtui::{
    app::{App, PlaybackState, Focus},
    audio::AudioPlayer,
    ui,
};
use ratatui::prelude::*;
use std::{io, time::Duration};

fn main() -> Result<()> {
    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state starting in current directory
    let mut app = App::default();
    // Initialize songs for starting directory
    app.update_songs();

    // Create audio player
    let mut audio_player = AudioPlayer::new()?;

    // Run app
    let res = run_app(&mut terminal, &mut app, &mut audio_player);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run_app<B: Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    audio_player: &mut AudioPlayer,
) -> Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Char('q') => return Ok(()),
                    KeyCode::Tab => {
                        if key.modifiers.contains(KeyModifiers::SHIFT) {
                            app.reverse_toggle_focus();
                        } else {
                            app.toggle_focus();
                        }
                    }
                    KeyCode::Char(' ') => {
                        match app.playback_state {
                            PlaybackState::Playing => {
                                audio_player.pause();
                                app.playback_state = PlaybackState::Paused;
                            }
                            PlaybackState::Paused => {
                                // Only resume if we have a current track
                                if app.current_track.is_some() {
                                    audio_player.resume();
                                    app.playback_state = PlaybackState::Playing;
                                }
                            }
                            PlaybackState::Stopped => {
                                // Try to play the current track if we have one
                                if let Some(track) = &app.current_track {
                                    if let Err(e) = audio_player.play(&track.path) {
                                        eprintln!("Error playing track: {}", e);
                                        continue;
                                    }
                                    app.playback_state = PlaybackState::Playing;
                                } else {
                                    // If no current track, try to play from current focus
                                    let tracks = match app.focus {
                                        Focus::Songs => &app.songs,
                                        Focus::Playlist => &app.playlist,
                                        _ => continue,
                                    };

                                    if !tracks.is_empty() {
                                        app.current_track_index = Some(0);
                                        app.current_track = tracks.first().cloned();
                                        if let Some(track) = &app.current_track {
                                            if let Err(e) = audio_player.play(&track.path) {
                                                eprintln!("Error playing track: {}", e);
                                                continue;
                                            }
                                            app.playback_state = PlaybackState::Playing;
                                        }
                                    }
                                }
                            }
                        }
                    }
                    KeyCode::Char('n') => {
                        app.next_track();
                        if let Some(track) = &app.current_track {
                            if let Err(e) = audio_player.play(&track.path) {
                                eprintln!("Error playing track: {}", e);
                                continue;
                            }
                            app.playback_state = PlaybackState::Playing;
                        }
                    }
                    KeyCode::Char('p') => {
                        app.previous_track();
                        if let Some(track) = &app.current_track {
                            if let Err(e) = audio_player.play(&track.path) {
                                eprintln!("Error playing track: {}", e);
                                continue;
                            }
                            app.playback_state = PlaybackState::Playing;
                        }
                    }
                    KeyCode::Char('+') => {
                        app.increase_volume();
                        audio_player.set_volume(app.volume as f32 / 100.0);
                    }
                    KeyCode::Char('-') => {
                        app.decrease_volume();
                        audio_player.set_volume(app.volume as f32 / 100.0);
                    }
                    KeyCode::Up | KeyCode::Char('k') => app.move_selection_up(),
                    KeyCode::Down | KeyCode::Char('j') => app.move_selection_down(),
                    KeyCode::Right => {
                        match app.focus {
                            Focus::Browser => {
                                if let Some(path) = app.filesystem.get_selected_path() {
                                    if path.is_dir() {
                                        if let Err(e) = app.enter_directory() {
                                            eprintln!("Error accessing directory: {}", e);
                                        }
                                    }
                                }
                            }
                            Focus::Songs => {
                                if key.modifiers.contains(KeyModifiers::SHIFT) {
                                    app.add_all_to_playlist();
                                } else {
                                    app.add_to_playlist();
                                }
                            }
                            _ => {}
                        }
                    }
                    KeyCode::Left => {
                        match app.focus {
                            Focus::Browser => app.go_to_parent(),
                            Focus::Songs => app.remove_from_playlist(),
                            Focus::Playlist => {
                                if key.modifiers.contains(KeyModifiers::SHIFT) {
                                    app.clear_playlist();
                                }
                            }
                        }
                    }
                    KeyCode::Enter => {
                        match app.focus {
                            Focus::Browser => {
                                if let Some(path) = app.filesystem.get_selected_path() {
                                    if path.is_dir() {
                                        if let Err(e) = app.enter_directory() {
                                            eprintln!("Error accessing directory: {}", e);
                                        }
                                    }
                                }
                            }
                            Focus::Songs | Focus::Playlist => {
                                app.play_selected();
                                if let Some(track) = &app.current_track {
                                    if let Err(e) = audio_player.play(&track.path) {
                                        eprintln!("Error playing track: {}", e);
                                        continue;
                                    }
                                }
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        if app.focus == Focus::Browser {
                            app.go_to_parent();
                        }
                    }
                    _ => {}
                }
            }
        }
    }
}
