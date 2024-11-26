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

pub trait StateManager {
    fn update(&mut self, action: Action) -> Option<Action>;
}

impl StateManager for AppState {
    fn update(&mut self, action: Action) -> Option<Action> {
        match action {
            // Navigation actions
            Action::NavigateUp | Action::NavigateDown | Action::Select | Action::Back => {
                // These actions are handled by the LibraryBrowser component
                Some(Action::Refresh)
            }
            Action::Refresh => None,

            // Direct playback control actions
            Action::Play => {
                self.player.playback_state = PlaybackState::Playing;
                None
            }
            Action::Pause => {
                self.player.playback_state = PlaybackState::Paused;
                None
            }
            Action::Stop => {
                self.player.playback_state = PlaybackState::Stopped;
                self.player.position = Duration::from_secs(0);
                None
            }
            Action::NextTrack => {
                if let Some(current_index) = self.playlist.selected_index {
                    let next_index = current_index + 1;
                    if next_index < self.playlist.tracks.len() {
                        return Some(Action::Playlist(crate::events::PlaylistAction::SelectTrack(next_index)));
                    }
                }
                None
            }
            Action::PreviousTrack => {
                if let Some(current_index) = self.playlist.selected_index {
                    if current_index > 0 {
                        return Some(Action::Playlist(crate::events::PlaylistAction::SelectTrack(current_index - 1)));
                    }
                }
                None
            }
            Action::VolumeUp => {
                let new_volume = (self.player.volume as u16 + 5).min(100) as u8;
                self.player.volume = new_volume;
                Some(Action::SetVolume(new_volume))
            }
            Action::VolumeDown => {
                let new_volume = self.player.volume.saturating_sub(5);
                self.player.volume = new_volume;
                Some(Action::SetVolume(new_volume))
            }
            Action::SetVolume(volume) => {
                self.player.volume = volume;
                None
            }

            // Nested action variants
            Action::Player(player_action) => {
                match player_action {
                    crate::events::PlayerAction::Play => {
                        self.player.playback_state = PlaybackState::Playing;
                        self.player.seek_state = SeekState::Normal;
                        None
                    }
                    crate::events::PlayerAction::Pause => {
                        self.player.playback_state = PlaybackState::Paused;
                        None
                    }
                    crate::events::PlayerAction::Stop => {
                        self.player.playback_state = PlaybackState::Stopped;
                        self.player.position = Duration::from_secs(0);
                        self.player.seek_state = SeekState::Normal;
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
                    crate::events::PlayerAction::Record => {
                        self.player.is_recording = !self.player.is_recording;
                        None
                    }
                    crate::events::PlayerAction::FastForward => {
                        self.player.seek_state = SeekState::FastForward;
                        None
                    }
                    crate::events::PlayerAction::Rewind => {
                        self.player.seek_state = SeekState::Rewind;
                        None
                    }
                    crate::events::PlayerAction::StopEject => {
                        self.player.playback_state = PlaybackState::Stopped;
                        self.player.position = Duration::from_secs(0);
                        self.player.current_track = None;
                        self.player.seek_state = SeekState::Normal;
                        Some(Action::Metadata(crate::events::MetadataAction::Clear))
                    }
                }
            }
            Action::Playlist(playlist_action) => {
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
                match ui_action {
                    crate::events::UIAction::Focus(direction) => {
                        self.ui.focused_component = match (self.ui.focused_component.as_str(), direction) {
                            ("library_browser", crate::events::FocusDirection::Next) => "track_list",
                            ("track_list", crate::events::FocusDirection::Next) => "track_details",
                            ("track_details", crate::events::FocusDirection::Next) => "current_track_info",
                            ("current_track_info", crate::events::FocusDirection::Next) => "playback_status",
                            ("playback_status", crate::events::FocusDirection::Next) => "controls",
                            ("controls", crate::events::FocusDirection::Next) => "volume_control",
                            ("volume_control", crate::events::FocusDirection::Next) => "library_browser",
                            
                            ("library_browser", crate::events::FocusDirection::Previous) => "volume_control",
                            ("track_list", crate::events::FocusDirection::Previous) => "library_browser",
                            ("track_details", crate::events::FocusDirection::Previous) => "track_list",
                            ("current_track_info", crate::events::FocusDirection::Previous) => "track_details",
                            ("playback_status", crate::events::FocusDirection::Previous) => "current_track_info",
                            ("controls", crate::events::FocusDirection::Previous) => "playback_status",
                            ("volume_control", crate::events::FocusDirection::Previous) => "controls",
                            
                            _ => "library_browser",
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
