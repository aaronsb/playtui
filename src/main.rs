use anyhow::Result;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use playtui::{
    app::{App, PlaybackState},
    audio::AudioPlayer,
    input_handler,
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
