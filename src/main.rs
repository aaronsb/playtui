use anyhow::Result;
use crossterm::{
    event::{
        self, DisableMouseCapture, EnableMouseCapture, Event as CrosstermEvent,
        KeyCode, KeyEventKind, KeyModifiers, MouseEvent as CrosstermMouseEvent,
        MouseEventKind, MouseButton,
    },
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::prelude::*;
use std::io;

use playtui::app::App;
use playtui::events::{Event, KeyEvent, MouseEvent, SystemEvent};
use playtui::ui;

fn map_key_event(code: KeyCode, modifiers: KeyModifiers) -> Option<KeyEvent> {
    match (code, modifiers) {
        (KeyCode::Char(' '), KeyModifiers::NONE) => Some(KeyEvent::Play), // Space for play/pause
        (KeyCode::Char('s'), KeyModifiers::NONE) => Some(KeyEvent::Stop),
        (KeyCode::Char('+'), _) => Some(KeyEvent::VolumeUp),
        (KeyCode::Char('-'), _) => Some(KeyEvent::VolumeDown),
        (KeyCode::Tab, KeyModifiers::SHIFT) => Some(KeyEvent::Focus(playtui::events::FocusDirection::Previous)),
        (KeyCode::Tab, KeyModifiers::NONE) => Some(KeyEvent::Focus(playtui::events::FocusDirection::Next)),
        (KeyCode::Right, KeyModifiers::NONE) => Some(KeyEvent::Next),
        (KeyCode::Left, KeyModifiers::NONE) => Some(KeyEvent::Previous),
        _ => None,
    }
}

fn map_mouse_event(mouse_event: CrosstermMouseEvent) -> Option<MouseEvent> {
    match mouse_event.kind {
        MouseEventKind::Down(button) if button == MouseButton::Left => Some(MouseEvent::Click {
            x: mouse_event.column as u16,
            y: mouse_event.row as u16,
        }),
        MouseEventKind::ScrollDown => Some(MouseEvent::Scroll { delta: -1 }),
        MouseEventKind::ScrollUp => Some(MouseEvent::Scroll { delta: 1 }),
        _ => None,
    }
}

fn main() -> Result<()> {
    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;

    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create app state
    let mut app = App::new();

    // Main loop
    loop {
        // Render UI
        terminal.draw(|frame| ui::render(frame, &app))?;
        
        // Update focus states before handling next event
        app.update_focus_states();

        // Handle events
        match event::read()? {
            CrosstermEvent::Key(key) => {
                if key.kind == KeyEventKind::Press {
                    // Check for quit condition first
                    if key.code == KeyCode::Char('q') {
                        break;
                    }

                    // Map and handle other key events
                    if let Some(key_event) = map_key_event(key.code, key.modifiers) {
                        if let Err(e) = app.handle_event(Event::Key(key_event)) {
                            // Log error but continue running
                            eprintln!("Error handling key event: {}", e);
                        }
                    }
                }
            }
            CrosstermEvent::Mouse(mouse) => {
                if let Some(mouse_event) = map_mouse_event(mouse) {
                    if let Err(e) = app.handle_event(Event::Mouse(mouse_event)) {
                        // Log error but continue running
                        eprintln!("Error handling mouse event: {}", e);
                    }
                }
            }
            CrosstermEvent::Resize(width, height) => {
                // Simply notify the app of the new terminal dimensions
                terminal.draw(|frame| ui::render(frame, &app))?;
            }
            _ => {}
        }
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
