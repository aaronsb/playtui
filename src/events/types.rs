use crossterm::event::KeyCode;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    System(SystemEvent),
    Navigation(NavigationEvent),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum KeyEvent {
    // Global Navigation Events (always available)
    Tab,
    BackTab,
    
    // Global Hotkeys (available regardless of focus)
    Quit,         // 'q'
    Escape,       // ESC key
    Space,        // Global pause/play
    
    // Frame-Specific Events (require focus)
    Enter,        // Activate selected item in focused frame
    Left,         // Navigate within focused frame
    Right,        // Navigate within focused frame
    Up,          // Navigate within focused frame
    Down,        // Navigate within focused frame
    
    // Other Navigation
    Home,
    End,
    PageUp,
    PageDown,
    
    // Direct Input
    Char(char),
    Backspace,
    Delete,
    
    // Focus Management
    Focus(FocusDirection),
    
    // Global Playback Controls
    Play,         // Direct play control
    Pause,        // Direct pause control
    Stop,         // Direct stop control
    Next,         // Direct next track
    Previous,     // Direct previous track
    VolumeUp,     // Direct volume up
    VolumeDown,   // Direct volume down
    Record,       // Direct record control
    FastForward,  // Direct fast forward
    Rewind,       // Direct rewind
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum NavigationEvent {
    Left,
    Right,
    Up,
    Down,
}

impl From<KeyCode> for KeyEvent {
    fn from(code: KeyCode) -> Self {
        match code {
            KeyCode::Char(c) => match c {
                ' ' => KeyEvent::Space,
                'q' | 'Q' => KeyEvent::Quit,
                'p' | 'P' => KeyEvent::Play,
                's' | 'S' => KeyEvent::Stop,
                'n' | 'N' => KeyEvent::Next,
                'b' | 'B' => KeyEvent::Previous,
                '+' => KeyEvent::VolumeUp,
                '-' => KeyEvent::VolumeDown,
                'u' | 'U' => KeyEvent::Pause,
                'r' | 'R' => KeyEvent::Record,
                'f' | 'F' => KeyEvent::FastForward,
                'w' | 'W' => KeyEvent::Rewind,
                _ => KeyEvent::Char(c),
            },
            KeyCode::Enter => KeyEvent::Enter,
            KeyCode::Left => KeyEvent::Left,
            KeyCode::Right => KeyEvent::Right,
            KeyCode::Up => KeyEvent::Up,
            KeyCode::Down => KeyEvent::Down,
            KeyCode::Tab => KeyEvent::Tab,
            KeyCode::BackTab => KeyEvent::BackTab,
            KeyCode::Esc => KeyEvent::Escape,
            KeyCode::Backspace => KeyEvent::Backspace,
            KeyCode::Delete => KeyEvent::Delete,
            KeyCode::Home => KeyEvent::Home,
            KeyCode::End => KeyEvent::End,
            KeyCode::PageUp => KeyEvent::PageUp,
            KeyCode::PageDown => KeyEvent::PageDown,
            _ => KeyEvent::Space, // Default case
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MouseEvent {
    Click { x: u16, y: u16 },
    Scroll { delta: i16 },
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SystemEvent {
    TrackEnded,
    TrackLoaded,
    Error, // Changed from String to unit variant for Copy
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FocusDirection {
    Next,
    Previous,
}
