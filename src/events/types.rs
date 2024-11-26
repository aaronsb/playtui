use crossterm::event::KeyCode;

#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    System(SystemEvent),
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

impl From<KeyCode> for KeyEvent {
    fn from(code: KeyCode) -> Self {
        match code {
            KeyCode::Char(c) => match c {
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
            _ => KeyEvent::Char(' '), // Default case
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
