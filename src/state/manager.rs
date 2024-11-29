use super::*;
use crate::events::{Action, PlayerAction, PlaylistAction, UIAction, MetadataAction, FocusDirection};

pub trait StateManager {
    fn update(&mut self, action: Action) -> Option<Action>;
}

impl StateManager for AppState {
    fn update(&mut self, action: Action) -> Option<Action> {
        match action {
            // Pass through key events without state changes
            Action::Key(_) => Some(Action::Refresh),
            
            // Navigation actions
            Action::NavigateUp | 
            Action::NavigateDown | 
            Action::NavigateLeft |
            Action::NavigateRight |
            Action::Select | 
            Action::Back => {
                // These actions are handled by the focused component
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
                        return Some(Action::Playlist(PlaylistAction::SelectTrack(next_index)));
                    }
                }
                None
            }
            Action::PreviousTrack => {
                if let Some(current_index) = self.playlist.selected_index {
                    if current_index > 0 {
                        return Some(Action::Playlist(PlaylistAction::SelectTrack(current_index - 1)));
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
                    PlayerAction::Play => {
                        self.player.playback_state = PlaybackState::Playing;
                        self.player.seek_state = SeekState::Normal;
                        None
                    }
                    PlayerAction::Pause => {
                        self.player.playback_state = PlaybackState::Paused;
                        None
                    }
                    PlayerAction::Stop => {
                        self.player.playback_state = PlaybackState::Stopped;
                        self.player.position = Duration::from_secs(0);
                        self.player.seek_state = SeekState::Normal;
                        None
                    }
                    PlayerAction::SetVolume(volume) => {
                        self.player.volume = volume;
                        None
                    }
                    PlayerAction::LoadTrack(path) => {
                        self.player.current_track = Some(path);
                        self.player.position = Duration::from_secs(0);
                        Some(Action::Metadata(MetadataAction::Load(
                            self.player.current_track.clone().unwrap(),
                        )))
                    }
                    PlayerAction::Record => {
                        self.player.is_recording = !self.player.is_recording;
                        None
                    }
                    PlayerAction::FastForward => {
                        self.player.seek_state = SeekState::FastForward;
                        None
                    }
                    PlayerAction::Rewind => {
                        self.player.seek_state = SeekState::Rewind;
                        None
                    }
                    PlayerAction::StopEject => {
                        self.player.playback_state = PlaybackState::Stopped;
                        self.player.position = Duration::from_secs(0);
                        self.player.current_track = None;
                        self.player.seek_state = SeekState::Normal;
                        Some(Action::Metadata(MetadataAction::Clear))
                    }
                }
            }
            Action::Playlist(playlist_action) => {
                match playlist_action {
                    PlaylistAction::SelectTrack(index) => {
                        self.playlist.selected_index = Some(index);
                        if let Some(track) = self.playlist.tracks.get(index) {
                            Some(Action::Player(PlayerAction::LoadTrack(
                                track.clone(),
                            )))
                        } else {
                            None
                        }
                    }
                    PlaylistAction::AddTrack(path) => {
                        self.playlist.tracks.push(path);
                        None
                    }
                    PlaylistAction::RemoveTrack(index) => {
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
                    PlaylistAction::Clear => {
                        self.playlist.tracks.clear();
                        self.playlist.selected_index = None;
                        None
                    }
                }
            }
            Action::UI(ui_action) => {
                match ui_action {
                    UIAction::Focus(direction) => {
                        self.ui.focused_component = match (self.ui.focused_component.as_str(), direction) {
                            ("library_browser", FocusDirection::Next) => "track_list",
                            ("track_list", FocusDirection::Next) => "track_details",
                            ("track_details", FocusDirection::Next) => "current_track_info",
                            ("current_track_info", FocusDirection::Next) => "playback_status",
                            ("playback_status", FocusDirection::Next) => "controls",
                            ("controls", FocusDirection::Next) => "volume_control",
                            ("volume_control", FocusDirection::Next) => "library_browser",
                            
                            ("library_browser", FocusDirection::Previous) => "volume_control",
                            ("track_list", FocusDirection::Previous) => "library_browser",
                            ("track_details", FocusDirection::Previous) => "track_list",
                            ("current_track_info", FocusDirection::Previous) => "track_details",
                            ("playback_status", FocusDirection::Previous) => "current_track_info",
                            ("controls", FocusDirection::Previous) => "playback_status",
                            ("volume_control", FocusDirection::Previous) => "controls",
                            
                            _ => "library_browser",
                        }
                        .to_string();
                        None
                    }
                    UIAction::UpdateTheme(theme) => {
                        self.ui.theme = theme;
                        None
                    }
                    UIAction::Resize { width, height } => {
                        self.ui.window_size = (width, height);
                        None
                    }
                }
            }
            Action::Metadata(metadata_action) => {
                match metadata_action {
                    MetadataAction::Load(path) => {
                        if let Some(metadata) = self.metadata.metadata_cache.get(&path) {
                            self.metadata.current_metadata = Some(metadata.clone());
                        }
                        None
                    }
                    MetadataAction::Update(metadata) => {
                        if let Some(current_track) = &self.player.current_track {
                            self.metadata.metadata_cache.insert(current_track.clone(), metadata.clone());
                            self.metadata.current_metadata = Some(metadata);
                        }
                        None
                    }
                    MetadataAction::Clear => {
                        self.metadata.current_metadata = None;
                        None
                    }
                }
            }
            Action::App(_) => None,
        }
    }
}
