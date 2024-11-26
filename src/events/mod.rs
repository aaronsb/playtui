use std::fmt;
use std::error::Error;

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

#[derive(Debug, Clone)]
pub enum Event {
    Key(KeyEvent),
    Mouse(MouseEvent),
    System(SystemEvent),
}

#[derive(Debug, Clone)]
pub enum KeyEvent {
    Play,
    Pause,
    Stop,
    Next,
    Previous,
    VolumeUp,
    VolumeDown,
    Focus(FocusDirection),
}

#[derive(Debug, Clone)]
pub enum MouseEvent {
    Click { x: u16, y: u16 },
    Scroll { delta: i16 },
}

#[derive(Debug, Clone)]
pub enum SystemEvent {
    TrackEnded,
    TrackLoaded,
    Error(String),
}

#[derive(Debug, Clone)]
pub enum FocusDirection {
    Next,
    Previous,
}

#[derive(Debug, Clone)]
pub enum Action {
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
            // Only dispatch to handlers that can handle this event type
            if handler.can_handle(event) {
                match handler.handle_event(event) {
                    Ok(Some(action)) => actions.push(action),
                    Ok(None) => continue,
                    Err(e) => {
                        // Log error but continue processing other handlers
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
