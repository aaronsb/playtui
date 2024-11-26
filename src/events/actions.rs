use super::types::FocusDirection;
use super::KeyEvent;

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    // Key events
    Key(KeyEvent),
    
    // Navigation actions
    NavigateUp,
    NavigateDown,
    NavigateLeft,
    NavigateRight,
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

#[derive(Debug, Clone, PartialEq)]
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

#[derive(Debug, Clone, PartialEq)]
pub enum PlaylistAction {
    SelectTrack(usize),
    AddTrack(String),
    RemoveTrack(usize),
    Clear,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UIAction {
    Focus(FocusDirection),
    UpdateTheme(String),
    Resize { width: u16, height: u16 },
}

#[derive(Debug, Clone, PartialEq)]
pub enum MetadataAction {
    Load(String),
    Update(TrackMetadata),
    Clear,
}

#[derive(Debug, Clone, PartialEq)]
pub enum AppAction {
    Quit,
    Cancel,
    Error(String),
    NoOp,
}

#[derive(Debug, Clone, PartialEq)]
pub struct TrackMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration: Option<u64>,
}
