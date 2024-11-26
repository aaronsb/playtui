use anyhow::Result;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event as CrosstermEvent,
        KeyCode, KeyEventKind, MouseEvent as CrosstermMouseEvent,
        MouseEventKind, MouseButton,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io::{self, BufWriter};
use std::fs::{OpenOptions, File};
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

use playtui::app::App;
use playtui::events::{Event, KeyEvent, MouseEvent};

fn log_raw_event(writer: &mut BufWriter<File>, event: &CrosstermEvent) -> std::io::Result<()> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    
    let log_entry = format!("[{}] Raw Event: {:?}\n", timestamp, event);
    writer.write_all(log_entry.as_bytes())?;
    writer.flush()?;
    
    Ok(())
}

fn main() -> Result<()> {
    // Create logs directory if it doesn't exist
    std::fs::create_dir_all("logs")?;

    // Open raw events log file with buffered writer
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let raw_log_file = format!("logs/raw_events_{}.log", timestamp);
    let file = OpenOptions::new()
        .create(true)
        .append(true)
        .write(true)
        .open(raw_log_file)?;
    let mut raw_logger = BufWriter::new(file);

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state with error handling for theme loading
    let mut app = match App::new() {
        Ok(app) => app,
        Err(e) => {
            // Clean up terminal before exiting
            disable_raw_mode()?;
            execute!(
                terminal.backend_mut(),
                LeaveAlternateScreen,
                DisableMouseCapture
            )?;
            terminal.show_cursor()?;
            
            // Return the error
            return Err(e);
        }
    };

    // Main loop
    loop {
        // Render UI
        terminal.draw(|frame| playtui::ui::render(frame, &mut app))?;

        // Handle events
        if event::poll(std::time::Duration::from_millis(50))? {
            match event::read()? {
                event @ CrosstermEvent::Key(key) => {
                    // Log raw event first, before any processing
                    if let Err(e) = log_raw_event(&mut raw_logger, &event) {
                        eprintln!("Error logging event: {}", e);
                    }

                    // Check for quit condition first
                    if key.code == KeyCode::Char('q') && key.kind == KeyEventKind::Press {
                        break;
                    }

                    // Convert key code to our internal KeyEvent type
                    let key_event = KeyEvent::from(key.code);
                    
                    // Handle the event
                    if let Err(e) = app.handle_event(Event::Key(key_event)) {
                        eprintln!("Error handling key event: {}", e);
                    }
                }
                event @ CrosstermEvent::Mouse(mouse) => {
                    // Log raw event first
                    if let Err(e) = log_raw_event(&mut raw_logger, &event) {
                        eprintln!("Error logging event: {}", e);
                    }

                    if let Some(mouse_event) = map_mouse_event(mouse) {
                        if let Err(e) = app.handle_event(Event::Mouse(mouse_event)) {
                            eprintln!("Error handling mouse event: {}", e);
                        }
                    }
                }
                event @ CrosstermEvent::Resize(_width, _height) => {
                    // Log raw event first
                    if let Err(e) = log_raw_event(&mut raw_logger, &event) {
                        eprintln!("Error logging event: {}", e);
                    }
                }
                event => {
                    // Log any other raw events
                    if let Err(e) = log_raw_event(&mut raw_logger, &event) {
                        eprintln!("Error logging event: {}", e);
                    }
                }
            }
        }
    }

    // Ensure final flush of logs before exit
    if let Err(e) = raw_logger.flush() {
        eprintln!("Error flushing final logs: {}", e);
    }

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn map_mouse_event(mouse_event: CrosstermMouseEvent) -> Option<MouseEvent> {
    match mouse_event.kind {
        MouseEventKind::Down(button) if button == MouseButton::Left => Some(MouseEvent::Click {
            x: mouse_event.column,
            y: mouse_event.row,
        }),
        MouseEventKind::ScrollDown => Some(MouseEvent::Scroll { delta: -1 }),
        MouseEventKind::ScrollUp => Some(MouseEvent::Scroll { delta: 1 }),
        _ => None,
    }
}
