use crossterm::event::KeyCode;
use std::path::PathBuf;
use crate::audio::AudioPlayer;
use crate::theme::Theme;

pub struct App {
    pub focused_pane: FocusedPane,
    pub file_list: FileList,
    pub playlist: Playlist,
    pub player_state: PlayerState,
    audio_player: Option<AudioPlayer>,
    pub theme: Theme,
    pub current_theme_name: String,
}

#[derive(Clone, Copy, PartialEq)]
pub enum FocusedPane {
    Files,
    Playlist,
    Controls,
}

pub struct FileList {
    pub current_dir: PathBuf,
    pub entries: Vec<PathBuf>,
    pub selected: usize,
}

pub struct Playlist {
    pub tracks: Vec<PathBuf>,
    pub selected: usize,
    pub current_track: Option<usize>,
}

pub struct PlayerState {
    pub is_playing: bool,
    pub volume: u8,
    pub current_position: Duration,
    pub total_duration: Duration,
}

use std::time::Duration;

impl App {
    pub fn new() -> Self {
        let mut file_list = FileList {
            current_dir: home::home_dir().unwrap_or_else(|| PathBuf::from("/")),
            entries: vec![],
            selected: 0,
        };
        file_list.refresh();

        // Try to create audio player
        let audio_player = match AudioPlayer::new() {
            Ok(player) => Some(player),
            Err(e) => {
                eprintln!("Failed to initialize audio: {}", e);
                None
            }
        };

        // Load default theme
        let (theme, theme_name) = match crate::theme::load_theme("monokai") {
            Ok(theme) => (theme, "monokai".to_string()),
            Err(_) => {
                // Fallback to a basic theme if loading fails
                eprintln!("Failed to load monokai theme, using fallback");
                (Self::create_fallback_theme(), "fallback".to_string())
            }
        };

        Self {
            focused_pane: FocusedPane::Files,
            file_list,
            playlist: Playlist {
                tracks: vec![],
                selected: 0,
                current_track: None,
            },
            player_state: PlayerState {
                is_playing: false,
                volume: 50,
                current_position: Duration::from_secs(0),
                total_duration: Duration::from_secs(0),
            },
            audio_player,
            theme,
            current_theme_name: theme_name,
        }
    }

    pub fn handle_key(&mut self, key: KeyCode) {
        // Global keys that work regardless of focus
        match key {
            KeyCode::Char(' ') => {
                self.toggle_playback();
                return;
            }
            KeyCode::Char('+') | KeyCode::Char('=') => {
                self.volume_up();
                return;
            }
            KeyCode::Char('-') | KeyCode::Char('_') => {
                self.volume_down();
                return;
            }
            KeyCode::Tab => {
                self.next_focus();
                return;
            }
            KeyCode::BackTab => {
                self.prev_focus();
                return;
            }
            KeyCode::Char('t') | KeyCode::Char('T') => {
                self.cycle_theme();
                return;
            }
            KeyCode::Left => {
                if self.focused_pane != FocusedPane::Files {
                    self.previous_track();
                    return;
                }
            }
            KeyCode::Right => {
                if self.focused_pane != FocusedPane::Files {
                    self.next_track();
                    return;
                }
            }
            _ => {}
        }

        // Context-specific keys based on focused pane
        match self.focused_pane {
            FocusedPane::Files => self.handle_file_keys(key),
            FocusedPane::Playlist => self.handle_playlist_keys(key),
            FocusedPane::Controls => {}, // Controls no longer focusable
        }
    }

    fn handle_file_keys(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => self.file_list.move_up(),
            KeyCode::Down => self.file_list.move_down(),
            KeyCode::Enter => {
                if let Some(path) = self.file_list.get_selected() {
                    if path.to_str() == Some("..") {
                        self.file_list.go_up_dir();
                    } else if path.is_dir() {
                        self.file_list.enter_dir();
                    } else if is_audio_file(path) {
                        self.playlist.tracks.push(path.clone());
                    }
                }
            },
            _ => {}
        }
    }

    fn handle_playlist_keys(&mut self, key: KeyCode) {
        match key {
            KeyCode::Up => self.playlist.move_up(),
            KeyCode::Down => self.playlist.move_down(),
            KeyCode::Enter => self.play_selected(),
            KeyCode::Delete | KeyCode::Char('d') => self.playlist.remove_selected(),
            KeyCode::Char('c') => self.clear_playlist(),
            _ => {}
        }
    }


    fn toggle_playback(&mut self) {
        if let Some(audio_player) = &mut self.audio_player {
            if self.player_state.is_playing {
                // Pause
                audio_player.pause();
                self.player_state.is_playing = false;
            } else {
                // Play or resume
                if self.playlist.current_track.is_some() {
                    audio_player.resume();
                    self.player_state.is_playing = true;
                } else {
                    // No track selected, try to play first track in playlist
                    if !self.playlist.tracks.is_empty() {
                        self.playlist.current_track = Some(0);
                        self.play_selected();
                    }
                }
            }
        }
    }

    fn volume_up(&mut self) {
        self.player_state.volume = (self.player_state.volume + 5).min(100);
        if let Some(audio_player) = &mut self.audio_player {
            audio_player.set_volume(self.player_state.volume as f32 / 100.0);
        }
    }

    fn volume_down(&mut self) {
        self.player_state.volume = self.player_state.volume.saturating_sub(5);
        if let Some(audio_player) = &mut self.audio_player {
            audio_player.set_volume(self.player_state.volume as f32 / 100.0);
        }
    }

    fn next_focus(&mut self) {
        self.focused_pane = match self.focused_pane {
            FocusedPane::Files => FocusedPane::Playlist,
            FocusedPane::Playlist => FocusedPane::Files,
            FocusedPane::Controls => FocusedPane::Files, // Just in case
        };
    }

    fn prev_focus(&mut self) {
        self.focused_pane = match self.focused_pane {
            FocusedPane::Files => FocusedPane::Playlist,
            FocusedPane::Playlist => FocusedPane::Files,
            FocusedPane::Controls => FocusedPane::Playlist, // Just in case
        };
    }


    fn play_selected(&mut self) {
        if !self.playlist.tracks.is_empty() {
            self.playlist.current_track = Some(self.playlist.selected);
            
            // Play the selected track
            if let Some(audio_player) = &mut self.audio_player {
                if let Some(track_idx) = self.playlist.current_track {
                    if let Some(track_path) = self.playlist.tracks.get(track_idx) {
                        match audio_player.play(track_path) {
                            Ok(_) => {
                                self.player_state.is_playing = true;
                                // Set volume to current level
                                audio_player.set_volume(self.player_state.volume as f32 / 100.0);
                                // Get track duration
                                self.player_state.total_duration = audio_player.get_duration();
                            }
                            Err(e) => {
                                eprintln!("Failed to play {}: {}", track_path.display(), e);
                                self.player_state.is_playing = false;
                            }
                        }
                    }
                }
            }
        }
    }

    fn next_track(&mut self) {
        if let Some(current) = self.playlist.current_track {
            if current + 1 < self.playlist.tracks.len() {
                self.playlist.current_track = Some(current + 1);
                self.playlist.selected = current + 1;
                self.play_selected();
            }
        } else if !self.playlist.tracks.is_empty() {
            self.playlist.current_track = Some(0);
            self.playlist.selected = 0;
            self.play_selected();
        }
    }

    fn previous_track(&mut self) {
        if let Some(current) = self.playlist.current_track {
            if current > 0 {
                self.playlist.current_track = Some(current - 1);
                self.playlist.selected = current - 1;
                self.play_selected();
            }
        }
    }

    pub fn switch_theme(&mut self, theme_name: &str) {
        match crate::theme::load_theme(theme_name) {
            Ok(theme) => {
                self.theme = theme;
                self.current_theme_name = theme_name.to_string();
            }
            Err(e) => {
                eprintln!("Failed to load theme '{}': {}", theme_name, e);
            }
        }
    }

    pub fn cycle_theme(&mut self) {
        let themes = crate::theme::list_available_themes();
        if themes.is_empty() {
            return;
        }

        let current_index = themes.iter().position(|t| t == &self.current_theme_name).unwrap_or(0);
        let next_index = (current_index + 1) % themes.len();
        self.switch_theme(&themes[next_index]);
    }

    fn create_fallback_theme() -> Theme {
        use crate::theme::{Theme, ThemeMetadata, ThemeColors, ThemeControls};
        use std::collections::HashMap;

        Theme {
            metadata: ThemeMetadata {
                name: "Fallback".to_string(),
                author: "PlayTUI".to_string(),
                version: "1.0".to_string(),
            },
            colors: ThemeColors {
                primary: "#00FFFF".to_string(),
                secondary: "#00FF00".to_string(),
                background: "#000000".to_string(),
                foreground: "#FFFFFF".to_string(),
                active: "#FFFF00".to_string(),
                inactive: "#808080".to_string(),
                playing: "#00FFFF".to_string(),
                error: "#FF0000".to_string(),
            },
            controls: ThemeControls::default(),
            styles: HashMap::new(),
        }
    }

    pub fn check_track_finished(&mut self) {
        if self.player_state.is_playing {
            if let Some(audio_player) = &self.audio_player {
                // Update current position
                self.player_state.current_position = audio_player.get_position();
                
                if audio_player.has_finished() {
                    // Track finished, advance to next
                    self.next_track();
                }
            }
        }
    }

    fn clear_playlist(&mut self) {
        // Stop current playback
        if let Some(audio_player) = &mut self.audio_player {
            audio_player.stop();
        }
        
        // Clear playlist
        self.playlist.tracks.clear();
        self.playlist.selected = 0;
        self.playlist.current_track = None;
        self.player_state.is_playing = false;
    }
}

impl FileList {
    fn refresh(&mut self) {
        self.entries.clear();
        
        // Add parent directory
        if let Some(_parent) = self.current_dir.parent() {
            self.entries.push(PathBuf::from(".."));
        }

        // Add directory contents
        if let Ok(entries) = std::fs::read_dir(&self.current_dir) {
            let mut paths: Vec<_> = entries
                .filter_map(|e| e.ok())
                .map(|e| e.path())
                .collect();
            
            // Sort directories first, then files
            paths.sort_by(|a, b| {
                match (a.is_dir(), b.is_dir()) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => a.file_name().cmp(&b.file_name()),
                }
            });
            
            self.entries.extend(paths);
        }
        
        self.selected = 0;
    }

    fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    fn move_down(&mut self) {
        if self.selected + 1 < self.entries.len() {
            self.selected += 1;
        }
    }

    fn enter_dir(&mut self) {
        if let Some(path) = self.get_selected() {
            if path.is_dir() || path.to_str() == Some("..") {
                if path.to_str() == Some("..") {
                    self.go_up_dir();
                } else {
                    self.current_dir = path.clone();
                    self.refresh();
                }
            }
        }
    }

    fn go_up_dir(&mut self) {
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
            self.refresh();
        }
    }

    fn get_selected(&self) -> Option<&PathBuf> {
        self.entries.get(self.selected)
    }
}

impl Playlist {
    fn move_up(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }

    fn move_down(&mut self) {
        if self.selected + 1 < self.tracks.len() {
            self.selected += 1;
        }
    }

    fn remove_selected(&mut self) {
        if self.selected < self.tracks.len() {
            self.tracks.remove(self.selected);
            if self.selected >= self.tracks.len() && self.selected > 0 {
                self.selected -= 1;
            }
        }
    }
}

fn is_audio_file(path: &PathBuf) -> bool {
    if let Some(ext) = path.extension() {
        matches!(
            ext.to_str().unwrap_or("").to_lowercase().as_str(),
            "mp3" | "flac" | "ogg" | "wav" | "m4a" | "aac"
        )
    } else {
        false
    }
}