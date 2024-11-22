use anyhow::Result;
use crossterm::event::{Event, KeyCode, KeyModifiers};
use crate::app::{App, PlaybackState, Focus, MenuPage, ThemeDirection};
use crate::audio::AudioPlayer;

pub fn handle_input(
    event: Event,
    app: &mut App,
    audio_player: &mut AudioPlayer,
) -> Result<Option<()>> {
    if let Event::Key(key) = event {
        // Handle menu-specific keys first
        if app.show_menu {
            match key.code {
                KeyCode::Char('m') | KeyCode::Char('q') => {
                    app.toggle_menu();
                }
                KeyCode::Tab => {
                    // In menu, tab always cycles forward regardless of shift
                    app.cycle_menu_page();
                }
                KeyCode::Up if app.menu_page == MenuPage::Looks => {
                    app.move_theme_selection(ThemeDirection::Up);
                }
                KeyCode::Down if app.menu_page == MenuPage::Looks => {
                    app.move_theme_selection(ThemeDirection::Down);
                }
                KeyCode::Left if app.menu_page == MenuPage::Looks => {
                    app.move_theme_selection(ThemeDirection::Left);
                }
                KeyCode::Right if app.menu_page == MenuPage::Looks => {
                    app.move_theme_selection(ThemeDirection::Right);
                }
                KeyCode::Enter if app.menu_page == MenuPage::Looks => {
                    if let Err(e) = app.apply_selected_theme() {
                        eprintln!("Error applying theme: {}", e);
                    } else {
                        // Save preferences after theme change
                        if let Err(e) = app.get_current_preferences().save() {
                            eprintln!("Error saving preferences: {}", e);
                        }
                    }
                }
                KeyCode::Char('r') if app.menu_page == MenuPage::Preferences => {
                    app.toggle_repeat_mode();
                    // Save preferences after repeat mode change
                    if let Err(e) = app.get_current_preferences().save() {
                        eprintln!("Error saving preferences: {}", e);
                    }
                }
                _ => {} // Ignore other keys when menu is shown
            }
            return Ok(None);
        }

        // Handle shift+tab separately to ensure it's caught
        if key.code == KeyCode::BackTab || (key.code == KeyCode::Tab && key.modifiers.contains(KeyModifiers::SHIFT)) {
            app.reverse_toggle_focus();
            return Ok(None);
        }

        match key.code {
            KeyCode::Char('q') => return Ok(Some(())),
            KeyCode::Char('m') => app.toggle_menu(),
            KeyCode::Tab => app.toggle_focus(),
            KeyCode::Char(' ') => handle_playback(app, audio_player)?,
            KeyCode::Char('.') => handle_next_track(app, audio_player)?,
            KeyCode::Char(',') => handle_previous_track(app, audio_player)?,
            KeyCode::Char('+') => {
                app.increase_volume();
                audio_player.set_volume(app.volume as f32 / 100.0);
                // Save preferences after volume change
                if let Err(e) = app.get_current_preferences().save() {
                    eprintln!("Error saving preferences: {}", e);
                }
            }
            KeyCode::Char('-') => {
                app.decrease_volume();
                audio_player.set_volume(app.volume as f32 / 100.0);
                // Save preferences after volume change
                if let Err(e) = app.get_current_preferences().save() {
                    eprintln!("Error saving preferences: {}", e);
                }
            }
            KeyCode::Up | KeyCode::Char('k') => app.move_selection_up(),
            KeyCode::Down | KeyCode::Char('j') => app.move_selection_down(),
            KeyCode::Right => handle_right_key(app, key.modifiers)?,
            KeyCode::Left => handle_left_key(app, key.modifiers)?,
            KeyCode::Enter => handle_enter_key(app, audio_player)?,
            KeyCode::Backspace => {
                if app.focus == Focus::Browser {
                    app.go_to_parent();
                    // Save preferences after directory change
                    if let Err(e) = app.get_current_preferences().save() {
                        eprintln!("Error saving preferences: {}", e);
                    }
                }
            }
            _ => {}
        }
    }
    Ok(None)
}

fn handle_playback(app: &mut App, audio_player: &mut AudioPlayer) -> Result<()> {
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
                audio_player.play(&track.path)?;
                app.playback_state = PlaybackState::Playing;
                app.playback_position = 0;
            } else {
                // If no current track, try to play from current focus
                let tracks = match app.focus {
                    Focus::Songs => &app.songs,
                    Focus::Playlist => &app.playlist,
                    _ => return Ok(()),
                };

                if !tracks.is_empty() {
                    app.current_track_index = Some(0);
                    app.current_track = tracks.first().cloned();
                    if let Some(track) = &app.current_track {
                        audio_player.play(&track.path)?;
                        app.playback_state = PlaybackState::Playing;
                        app.playback_position = 0;
                    }
                }
            }
        }
    }
    Ok(())
}

fn handle_next_track(app: &mut App, audio_player: &mut AudioPlayer) -> Result<()> {
    app.next_track();
    if let Some(track) = &app.current_track {
        audio_player.play(&track.path)?;
        app.playback_state = PlaybackState::Playing;
        app.playback_position = 0;
    }
    Ok(())
}

fn handle_previous_track(app: &mut App, audio_player: &mut AudioPlayer) -> Result<()> {
    app.previous_track();
    if let Some(track) = &app.current_track {
        audio_player.play(&track.path)?;
        app.playback_state = PlaybackState::Playing;
        app.playback_position = 0;
    }
    Ok(())
}

fn handle_right_key(app: &mut App, modifiers: KeyModifiers) -> Result<()> {
    match app.focus {
        Focus::Browser => {
            if let Some(path) = app.filesystem.get_selected_path() {
                if path.is_dir() {
                    if let Err(e) = app.enter_directory() {
                        eprintln!("Error accessing directory: {}", e);
                    } else {
                        // Save preferences after directory change
                        if let Err(e) = app.get_current_preferences().save() {
                            eprintln!("Error saving preferences: {}", e);
                        }
                    }
                }
            }
        }
        Focus::Songs => {
            if modifiers.contains(KeyModifiers::SHIFT) {
                app.add_all_to_playlist();
            } else {
                app.add_to_playlist();
            }
        }
        _ => {}
    }
    Ok(())
}

fn handle_left_key(app: &mut App, modifiers: KeyModifiers) -> Result<()> {
    match app.focus {
        Focus::Browser => {
            app.go_to_parent();
            // Save preferences after directory change
            if let Err(e) = app.get_current_preferences().save() {
                eprintln!("Error saving preferences: {}", e);
            }
        }
        Focus::Songs => app.remove_from_playlist(),
        Focus::Playlist => {
            if modifiers.contains(KeyModifiers::SHIFT) {
                app.clear_playlist();
            }
        }
    }
    Ok(())
}

fn handle_enter_key(app: &mut App, audio_player: &mut AudioPlayer) -> Result<()> {
    match app.focus {
        Focus::Browser => {
            if let Some(path) = app.filesystem.get_selected_path() {
                if path.is_dir() {
                    if let Err(e) = app.enter_directory() {
                        eprintln!("Error accessing directory: {}", e);
                    } else {
                        // Save preferences after directory change
                        if let Err(e) = app.get_current_preferences().save() {
                            eprintln!("Error saving preferences: {}", e);
                        }
                    }
                }
            }
        }
        Focus::Songs | Focus::Playlist => {
            app.play_selected();
            if let Some(track) = &app.current_track {
                audio_player.play(&track.path)?;
                app.playback_position = 0;
            }
        }
    }
    Ok(())
}
