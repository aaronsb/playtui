use std::time::Duration;
use crate::events::{Action, TrackMetadata};

#[derive(Debug, Clone)]
pub enum PlaybackState {
    Playing,
    Paused,
    Stopped,
}

#[derive(Debug, Clone)]
pub struct PlayerState {
    pub playback_state: PlaybackState,
    pub volume: u8,
    pub position: Duration,
    pub current_track: Option<String>,
}

impl Default for PlayerState {
    fn default() -> Self {
        Self {
            playback_state: PlaybackState::Stopped,
            volume: 100,
            position: Duration::from_secs(0),
            current_track: None,
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
            focused_component: "playlist".to_string(),
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
pub struct AppState {
    pub player: PlayerState,
    pub playlist: PlaylistState,
    pub ui: UIState,
    pub metadata: MetadataState,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            player: PlayerState::default(),
            playlist: PlaylistState::default(),
            ui: UIState::default(),
            metadata: MetadataState::default(),
        }
    }
}

pub trait StateManager {
    fn update(&mut self, action: Action) -> Option<Action>;
}

impl StateManager for AppState {
    fn update(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::Player(player_action) => {
                // Handle player actions
                match player_action {
                    crate::events::PlayerAction::Play => {
                        self.player.playback_state = PlaybackState::Playing;
                        None
                    }
                    crate::events::PlayerAction::Pause => {
                        self.player.playback_state = PlaybackState::Paused;
                        None
                    }
                    crate::events::PlayerAction::Stop => {
                        self.player.playback_state = PlaybackState::Stopped;
                        self.player.position = Duration::from_secs(0);
                        None
                    }
                    crate::events::PlayerAction::SetVolume(volume) => {
                        self.player.volume = volume;
                        None
                    }
                    crate::events::PlayerAction::LoadTrack(path) => {
                        self.player.current_track = Some(path);
                        self.player.position = Duration::from_secs(0);
                        Some(Action::Metadata(crate::events::MetadataAction::Load(
                            self.player.current_track.clone().unwrap(),
                        )))
                    }
                }
            }
            Action::Playlist(playlist_action) => {
                // Handle playlist actions
                match playlist_action {
                    crate::events::PlaylistAction::SelectTrack(index) => {
                        self.playlist.selected_index = Some(index);
                        if let Some(track) = self.playlist.tracks.get(index) {
                            Some(Action::Player(crate::events::PlayerAction::LoadTrack(
                                track.clone(),
                            )))
                        } else {
                            None
                        }
                    }
                    crate::events::PlaylistAction::AddTrack(path) => {
                        self.playlist.tracks.push(path);
                        None
                    }
                    crate::events::PlaylistAction::RemoveTrack(index) => {
                        if index < self.playlist.tracks.len() {
                            self.playlist.tracks.remove(index);
                            // Adjust selected index if necessary
                            if let Some(selected) = self.playlist.selected_index {
                                if selected >= index {
                                    self.playlist.selected_index = if selected > 0 {
                                        Some(selected - 1)
                                    } else {
                                        None
                                    };
                                }
                            }
                        }
                        None
                    }
                    crate::events::PlaylistAction::Clear => {
                        self.playlist.tracks.clear();
                        self.playlist.selected_index = None;
                        None
                    }
                }
            }
            Action::UI(ui_action) => {
                // Handle UI actions
                match ui_action {
                    crate::events::UIAction::Focus(direction) => {
                        // Update focused component based on direction
                        self.ui.focused_component = match (self.ui.focused_component.as_str(), direction) {
                            ("playlist", crate::events::FocusDirection::Next) => "nowplaying",
                            ("playlist", crate::events::FocusDirection::Previous) => "controls",
                            ("nowplaying", crate::events::FocusDirection::Next) => "controls",
                            ("nowplaying", crate::events::FocusDirection::Previous) => "playlist",
                            ("controls", crate::events::FocusDirection::Next) => "playlist",
                            ("controls", crate::events::FocusDirection::Previous) => "nowplaying",
                            _ => "playlist",
                        }
                        .to_string();
                        None
                    }
                    crate::events::UIAction::UpdateTheme(theme) => {
                        self.ui.theme = theme;
                        None
                    }
                    crate::events::UIAction::Resize { width, height } => {
                        self.ui.window_size = (width, height);
                        None
                    }
                }
            }
            Action::Metadata(metadata_action) => {
                // Handle metadata actions
                match metadata_action {
                    crate::events::MetadataAction::Load(path) => {
                        if let Some(metadata) = self.metadata.metadata_cache.get(&path) {
                            self.metadata.current_metadata = Some(metadata.clone());
                        }
                        None
                    }
                    crate::events::MetadataAction::Update(metadata) => {
                        if let Some(current_track) = &self.player.current_track {
                            self.metadata.metadata_cache.insert(current_track.clone(), metadata.clone());
                            self.metadata.current_metadata = Some(metadata);
                        }
                        None
                    }
                    crate::events::MetadataAction::Clear => {
                        self.metadata.current_metadata = None;
                        None
                    }
                }
            }
            Action::App(_) => None,
        }
    }
}
