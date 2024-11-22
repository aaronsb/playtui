use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    style::{Color, SetForegroundColor},
};
use playtui::{
    app::{App, PlaybackState},
    audio::AudioPlayer,
    input_handler,
    preferences::{Preferences, PreferencesError},
    ui,
};
use ratatui::prelude::*;
use std::{io::{self, Write}, time::Duration};

fn handle_preferences_error(error: PreferencesError) -> Result<Preferences> {
    println!("{}", SetForegroundColor(Color::Yellow));
    println!("Warning: {}", error);
    println!("Choose an option:");
    println!("1. Re-create default configuration");
    println!("2. Continue without preferences");
    println!("3. Exit");
    println!("{}", SetForegroundColor(Color::Reset));
    
    io::stdout().flush()?;

    loop {
        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Char('1') => {
                    let prefs = Preferences::default();
                    prefs.save()?;
                    return Ok(prefs);
                }
                KeyCode::Char('2') => {
                    return Ok(Preferences::default());
                }
                KeyCode::Char('3') => {
                    std::process::exit(0);
                }
                _ => {}
            }
        }
    }
}

fn main() -> Result<()> {
    // Load preferences before setting up the terminal
    let (preferences, error) = Preferences::load()?;
    let preferences = if let Some(error) = error {
        handle_preferences_error(error)?
    } else {
        preferences
    };

    // Set up terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state with loaded preferences
    let mut app = App::default();
    // Apply preferences
    app.apply_preferences(&preferences);
    // Initialize songs for starting directory
    app.update_songs();

    // Create audio player
    let mut audio_player = AudioPlayer::new()?;
    // Apply audio preferences
    audio_player.set_volume(preferences.volume);

    // Run app
    let res = run_app(&mut terminal, &mut app, &mut audio_player);

    // Save preferences before exit
    if let Err(e) = preferences.save() {
        eprintln!("Failed to save preferences: {}", e);
    }

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
        // Update playback position if playing
        if app.playback_state == PlaybackState::Playing {
            app.playback_position = audio_player.position();
        }

        terminal.draw(|f| ui::draw(f, app))?;

        if event::poll(Duration::from_millis(100))? {
            let event = event::read()?;
            if let Some(()) = input_handler::handle_input(event, app, audio_player)? {
                return Ok(());
            }
        }

        // Check if playback has finished
        if app.playback_state == PlaybackState::Playing && !audio_player.is_playing() {
            app.stop_playback();
            audio_player.stop();
        }
    }
}
