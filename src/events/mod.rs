use std::fmt;
use std::error::Error;
use crossterm::event::KeyCode;

#[derive(Debug)]
pub enum EventError {
    InvalidEvent(String),
    HandlerError(String),
    DispatchError(String),
}

impl fmt::Display for EventError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EventError::InvalidEvent(msg) => write!(f, "Invalid event: {}", msg),
            EventError::HandlerError(msg) => write!(f, "Handler error: {}", msg),
            EventError::DispatchError(msg) => write!(f, "Dispatch error: {}", msg),
        }
    }
}

impl Error for EventError {}

pub type EventResult<T> = Result<T, EventError>;

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

#[derive(Debug, Clone, PartialEq)]
pub enum FocusDirection {
    Next,
    Previous,
}

#[derive(Debug, Clone)]
pub enum Action {
    // Navigation actions
    NavigateUp,
    NavigateDown,
    Select,
    Back,
    Refresh,
    
    // Playback control actions
    Play,
    Pause,
    Stop,
    NextTrack,
    PreviousTrack,
    VolumeUp,
    VolumeDown,
    SetVolume(u8),
    
    // Player state actions
    Player(PlayerAction),
    Playlist(PlaylistAction),
    UI(UIAction),
    Metadata(MetadataAction),
    App(AppAction),
}

#[derive(Debug, Clone)]
pub enum PlayerAction {
    Play,
    Pause,
    Stop,
    SetVolume(u8),
    LoadTrack(String),
    // New player actions
    Record,
    FastForward,
    Rewind,
    StopEject,
}

#[derive(Debug, Clone)]
pub enum PlaylistAction {
    SelectTrack(usize),
    AddTrack(String),
    RemoveTrack(usize),
    Clear,
}

#[derive(Debug, Clone)]
pub enum UIAction {
    Focus(FocusDirection),
    UpdateTheme(String),
    Resize { width: u16, height: u16 },
}

#[derive(Debug, Clone)]
pub enum MetadataAction {
    Load(String),
    Update(TrackMetadata),
    Clear,
}

#[derive(Debug, Clone)]
pub enum AppAction {
    Quit,
    Error(String),
}

#[derive(Debug, Clone)]
pub struct TrackMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration: Option<u64>,
}

pub trait EventHandler {
    fn handle_event(&mut self, event: &Event) -> EventResult<Option<Action>>;
    fn can_handle(&self, event: &Event) -> bool;
}

pub struct EventDispatcher {
    handlers: Vec<Box<dyn EventHandler>>,
}

impl EventDispatcher {
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    pub fn register_handler(&mut self, handler: Box<dyn EventHandler>) {
        self.handlers.push(handler);
    }

    pub fn dispatch(&mut self, event: &Event) -> EventResult<Vec<Action>> {
        let mut actions = Vec::new();
        
        for handler in self.handlers.iter_mut() {
            if handler.can_handle(event) {
                match handler.handle_event(event) {
                    Ok(Some(action)) => actions.push(action),
                    Ok(None) => continue,
                    Err(e) => {
                        eprintln!("Handler error: {}", e);
                        continue;
                    }
                }
            }
        }

        Ok(actions)
    }

    pub fn dispatch_filtered<F>(&mut self, event: &Event, filter: F) -> EventResult<Vec<Action>>
    where
        F: Fn(&dyn EventHandler) -> bool,
    {
        let mut actions = Vec::new();
        
        for handler in self.handlers.iter_mut() {
            if filter(handler.as_ref()) && handler.can_handle(event) {
                match handler.handle_event(event) {
                    Ok(Some(action)) => actions.push(action),
                    Ok(None) => continue,
                    Err(e) => {
                        eprintln!("Handler error: {}", e);
                        continue;
                    }
                }
            }
        }

        Ok(actions)
    }
}

// Helper trait for type-based event filtering
pub trait EventFilter {
    fn accepts(&self, event: &Event) -> bool;
}

// Implement EventFilter for common event types
impl EventFilter for KeyEvent {
    fn accepts(&self, event: &Event) -> bool {
        matches!(event, Event::Key(_))
    }
}

impl EventFilter for MouseEvent {
    fn accepts(&self, event: &Event) -> bool {
        matches!(event, Event::Mouse(_))
    }
}

impl EventFilter for SystemEvent {
    fn accepts(&self, event: &Event) -> bool {
        matches!(event, Event::System(_))
    }
}
