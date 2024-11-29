mod manager;
mod transitions;
mod history;
mod validation;

pub use manager::StateManager;
pub use transitions::{StateTransition, AppStateTransition};
pub use history::{StateHistory, AppStateHistory};
pub use validation::{StateValidator, AppStateValidator};

use std::time::Duration;
use crate::events::{Action, TrackMetadata};

#[derive(Debug, Clone)]
pub enum PlaybackState {
    Playing,
    Paused,
    Stopped,
}

#[derive(Debug, Clone)]
pub enum SeekState {
    Normal,
    FastForward,
    Rewind,
}

#[derive(Debug, Clone)]
pub struct PlayerState {
    pub playback_state: PlaybackState,
    pub volume: u8,
    pub position: Duration,
    pub current_track: Option<String>,
    pub is_recording: bool,
    pub seek_state: SeekState,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            playback_state: PlaybackState::Stopped,
            volume: 100,
            position: Duration::from_secs(0),
            current_track: None,
            is_recording: false,
            seek_state: SeekState::Normal,
        }
    }
}

#[derive(Debug, Clone)]
pub struct PlaylistState {
    pub tracks: Vec<String>,
    pub selected_index: Option<usize>,
}

impl Default for PlaylistState {
    fn default() -> Self {
        Self {
            tracks: Vec::new(),
            selected_index: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct UIState {
    pub theme: String,
    pub focused_component: String,
    pub window_size: (u16, u16),
}

impl Default for UIState {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
            focused_component: "library_browser".to_string(),
            window_size: (0, 0),
        }
    }
}

#[derive(Debug, Clone)]
pub struct MetadataState {
    pub current_metadata: Option<TrackMetadata>,
    pub metadata_cache: std::collections::HashMap<String, TrackMetadata>,
}

impl Default for MetadataState {
    fn default() -> Self {
        Self {
            current_metadata: None,
            metadata_cache: std::collections::HashMap::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct NavigationState {
    pub current_path: Option<String>,
    pub selected_item: Option<String>,
}

impl Default for NavigationState {
    fn default() -> Self {
        Self {
            current_path: None,
            selected_item: None,
        }
    }
}

#[derive(Debug, Clone)]
pub struct AppState {
    pub player: PlayerState,
    pub playlist: PlaylistState,
    pub ui: UIState,
    pub metadata: MetadataState,
    pub navigation: NavigationState,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            player: PlayerState::default(),
            playlist: PlaylistState::default(),
            ui: UIState::default(),
            metadata: MetadataState::default(),
            navigation: NavigationState::default(),
        }
    }
}
