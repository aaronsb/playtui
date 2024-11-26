use crossterm::event::KeyCode;

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    System(SystemEvent),
    Navigation(NavigationEvent),
}

#[derive(Debug, Clone, PartialEq)]
pub enum KeyEvent {
    Char(char),
    Enter,
    Space,
    Left,
    Right,
    Up,
    Down,
    Tab,
    BackTab,
    Esc,
    Backspace,
    Delete,
    Home,
    End,
    PageUp,
    PageDown,
    Focus(FocusDirection),
    // Playback controls
    Play,
    Pause,
    Stop,
    Next,
    Previous,
    VolumeUp,
    VolumeDown,
    // New control events
    Record,
    FastForward,
    Rewind,
}

#[derive(Debug, Clone, PartialEq)]
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
            KeyCode::Esc => KeyEvent::Esc,
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

#[derive(Debug, Clone, PartialEq)]
pub enum MouseEvent {
    Click { x: u16, y: u16 },
    Scroll { delta: i16 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum SystemEvent {
    TrackEnded,
    TrackLoaded,
    Error(String),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FocusDirection {
    Next,
    Previous,
}
