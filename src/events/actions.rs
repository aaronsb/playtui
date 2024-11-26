use super::types::FocusDirection;

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
    NoOp,
}

#[derive(Debug, Clone)]
pub struct TrackMetadata {
    pub title: Option<String>,
    pub artist: Option<String>,
    pub album: Option<String>,
    pub duration: Option<u64>,
}
