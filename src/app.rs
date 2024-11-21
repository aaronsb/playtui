use std::path::PathBuf;
use crate::theme::Theme;

#[derive(Debug, Clone, PartialEq)]
pub struct Track {
    pub path: PathBuf,
    pub title: Option<String>,
    pub artist: Option<String>,
    pub duration: Option<u64>,
}

#[derive(Debug, PartialEq)]
pub enum PlaybackState {
    Playing,
    Paused,
    Stopped,
}

#[derive(Debug, PartialEq)]
pub enum Focus {
    Browser,
    Songs,
    Playlist,
}

#[derive(Debug)]
pub struct FileSystem {
    pub current_dir: PathBuf,
    pub entries: Vec<PathBuf>,
    pub selected_entry: usize,
}

impl FileSystem {
    pub fn new(path: PathBuf) -> Self {
        let mut fs = Self {
            current_dir: path,
            entries: Vec::new(),
            selected_entry: 0,
        };
        fs.refresh_entries();
        fs
    }

    pub fn refresh_entries(&mut self) {
        let mut entries = Vec::new();

        // Add parent directory (..) if not at root
        if let Some(parent) = self.current_dir.parent() {
            entries.push(parent.to_path_buf());
        }

        // Add immediate subdirectories only
        if let Ok(read_dir) = std::fs::read_dir(&self.current_dir) {
            let mut dirs: Vec<_> = read_dir
                .filter_map(Result::ok)
                .filter(|entry| {
                    entry.file_type()
                        .map(|ft| ft.is_dir())
                        .unwrap_or(false)
                })
                .map(|entry| entry.path())
                .collect();
            dirs.sort();
            entries.extend(dirs);
        }

        self.entries = entries;
        self.selected_entry = 0;
    }

    pub fn navigate_up(&mut self) {
        if self.selected_entry > 0 {
            self.selected_entry -= 1;
        }
    }

    pub fn navigate_down(&mut self) {
        if self.selected_entry + 1 < self.entries.len() {
            self.selected_entry += 1;
        }
    }

    pub fn enter_directory(&mut self) -> std::io::Result<()> {
        if let Some(path) = self.entries.get(self.selected_entry).cloned() {
            if path.is_dir() {
                // Try to read the directory first to check permissions
                std::fs::read_dir(&path)?;
                self.current_dir = path;
                self.refresh_entries();
            }
        }
        Ok(())
    }

    pub fn go_to_parent(&mut self) {
        if let Some(parent) = self.current_dir.parent() {
            self.current_dir = parent.to_path_buf();
            self.refresh_entries();
        }
    }

    pub fn get_current_dir_display(&self) -> String {
        if let Ok(canonical) = self.current_dir.canonicalize() {
            if let Some(name) = canonical.file_name() {
                if let Some(name_str) = name.to_str() {
                    return name_str.to_string();
                }
            }
        }
        ".".to_string()
    }

    pub fn get_entry_name(&self, index: usize) -> String {
        if let Some(path) = self.entries.get(index) {
            if let Some(parent) = self.current_dir.parent() {
                if path == parent {
                    return "..".to_string();
                }
            }

            if let Some(name) = path.file_name() {
                if let Some(name_str) = name.to_str() {
                    return name_str.to_string();
                }
            }
        }
        "Unknown".to_string()
    }

    pub fn get_selected_path(&self) -> Option<PathBuf> {
        self.entries.get(self.selected_entry).cloned()
    }
}

#[derive(Debug)]
pub struct App {
    pub songs: Vec<Track>,
    pub playlist: Vec<Track>,
    pub current_track_index: Option<usize>,
    pub current_track: Option<Track>,
    pub playback_state: PlaybackState,
    pub volume: u8,
    pub selected_song_index: usize,
    pub selected_playlist_index: usize,
    pub filesystem: FileSystem,
    pub focus: Focus,
    pub show_menu: bool,
    pub playback_position: u64,
    pub theme: Theme,
}

impl Default for App {
    fn default() -> Self {
        // Start in the parent directory
        let current_dir = PathBuf::from("..").canonicalize().unwrap_or_else(|_| PathBuf::from("."));
        
        // Load theme
        let theme = Theme::load().expect("Failed to load theme");

        Self {
            songs: Vec::new(),
            playlist: Vec::new(),
            current_track_index: None,
            current_track: None,
            playback_state: PlaybackState::Stopped,
            volume: 50,
            selected_song_index: 0,
            selected_playlist_index: 0,
            filesystem: FileSystem::new(current_dir),
            focus: Focus::Browser,
            show_menu: false,
            playback_position: 0,
            theme,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn toggle_menu(&mut self) {
        self.show_menu = !self.show_menu;
    }

    pub fn next_track(&mut self) {
        if let Some(current_index) = self.current_track_index {
            let tracks = match self.focus {
                Focus::Songs => &self.songs,
                Focus::Playlist => &self.playlist,
                _ => return,
            };
            
            if current_index + 1 < tracks.len() {
                self.current_track_index = Some(current_index + 1);
                if let Some(track) = tracks.get(current_index + 1) {
                    self.current_track = Some(track.clone());
                    self.playback_position = 0;
                }
            }
        } else {
            match self.focus {
                Focus::Songs if !self.songs.is_empty() => {
                    self.current_track_index = Some(0);
                    self.current_track = self.songs.first().cloned();
                    self.playback_position = 0;
                }
                Focus::Playlist if !self.playlist.is_empty() => {
                    self.current_track_index = Some(0);
                    self.current_track = self.playlist.first().cloned();
                    self.playback_position = 0;
                }
                _ => {}
            }
        }
    }

    pub fn previous_track(&mut self) {
        if let Some(current_index) = self.current_track_index {
            if current_index > 0 {
                self.current_track_index = Some(current_index - 1);
                let tracks = match self.focus {
                    Focus::Songs => &self.songs,
                    Focus::Playlist => &self.playlist,
                    _ => return,
                };
                if let Some(track) = tracks.get(current_index - 1) {
                    self.current_track = Some(track.clone());
                    self.playback_position = 0;
                }
            }
        }
    }

    pub fn toggle_playback(&mut self) {
        match self.playback_state {
            PlaybackState::Playing => {
                self.playback_state = PlaybackState::Paused;
            }
            PlaybackState::Paused => {
                // Only resume if we have a current track
                if self.current_track.is_some() {
                    self.playback_state = PlaybackState::Playing;
                }
            }
            PlaybackState::Stopped => {
                // Only start playing if we have a track selected
                if self.current_track.is_some() {
                    self.playback_state = PlaybackState::Playing;
                    self.playback_position = 0;
                }
            }
        }
    }

    pub fn stop_playback(&mut self) {
        self.playback_state = PlaybackState::Stopped;
        self.current_track = None;
        self.current_track_index = None;
        self.playback_position = 0;
    }

    pub fn increase_volume(&mut self) {
        self.volume = self.volume.saturating_add(5).min(100);
    }

    pub fn decrease_volume(&mut self) {
        self.volume = self.volume.saturating_sub(5);
    }

    pub fn move_selection_up(&mut self) {
        match self.focus {
            Focus::Browser => self.filesystem.navigate_up(),
            Focus::Songs => {
                if self.selected_song_index > 0 {
                    self.selected_song_index -= 1;
                }
            }
            Focus::Playlist => {
                if self.selected_playlist_index > 0 {
                    self.selected_playlist_index -= 1;
                }
            }
        }
    }

    pub fn move_selection_down(&mut self) {
        match self.focus {
            Focus::Browser => self.filesystem.navigate_down(),
            Focus::Songs => {
                if self.selected_song_index + 1 < self.songs.len() {
                    self.selected_song_index += 1;
                }
            }
            Focus::Playlist => {
                if self.selected_playlist_index + 1 < self.playlist.len() {
                    self.selected_playlist_index += 1;
                }
            }
        }
    }

    pub fn toggle_focus(&mut self) {
        self.focus = match self.focus {
            Focus::Browser => Focus::Songs,
            Focus::Songs => Focus::Playlist,
            Focus::Playlist => Focus::Browser,
        };
    }

    pub fn reverse_toggle_focus(&mut self) {
        self.focus = match self.focus {
            Focus::Browser => Focus::Playlist,
            Focus::Songs => Focus::Browser,
            Focus::Playlist => Focus::Songs,
        };
    }

    pub fn play_selected(&mut self) {
        match self.focus {
            Focus::Songs => {
                if !self.songs.is_empty() {
                    self.current_track_index = Some(self.selected_song_index);
                    self.current_track = self.songs.get(self.selected_song_index).cloned();
                    self.playback_state = PlaybackState::Playing;
                    self.playback_position = 0;
                }
            }
            Focus::Playlist => {
                if !self.playlist.is_empty() {
                    self.current_track_index = Some(self.selected_playlist_index);
                    self.current_track = self.playlist.get(self.selected_playlist_index).cloned();
                    self.playback_state = PlaybackState::Playing;
                    self.playback_position = 0;
                }
            }
            _ => {}
        }
    }

    pub fn update_songs(&mut self) {
        if let Ok(tracks) = crate::files::scan_directory(&self.filesystem.current_dir) {
            self.songs = tracks;
            self.selected_song_index = 0;
            // Don't reset current track or playback state when changing directories
        }
    }

    pub fn enter_directory(&mut self) -> std::io::Result<()> {
        self.filesystem.enter_directory()?;
        self.update_songs();
        Ok(())
    }

    pub fn go_to_parent(&mut self) {
        self.filesystem.go_to_parent();
        self.update_songs();
    }

    pub fn add_to_playlist(&mut self) {
        if self.focus == Focus::Songs && !self.songs.is_empty() {
            if let Some(track) = self.songs.get(self.selected_song_index) {
                self.playlist.push(track.clone());
            }
        }
    }

    pub fn add_all_to_playlist(&mut self) {
        if self.focus == Focus::Songs {
            // Only add songs that aren't already in the playlist
            let new_songs: Vec<Track> = self.songs
                .iter()
                .filter(|track| !self.playlist.contains(track))
                .cloned()
                .collect();
            self.playlist.extend(new_songs);
        }
    }

    pub fn remove_from_playlist(&mut self) {
        if self.focus == Focus::Songs && !self.playlist.is_empty() {
            if let Some(track) = self.songs.get(self.selected_song_index) {
                // Find the last occurrence of this track in the playlist
                if let Some(index) = self.playlist.iter().rposition(|t| t.path == track.path) {
                    self.playlist.remove(index);
                    // Adjust selected_playlist_index if necessary
                    if self.selected_playlist_index >= self.playlist.len() {
                        self.selected_playlist_index = self.playlist.len().saturating_sub(1);
                    }
                }
            }
        }
    }

    pub fn clear_playlist(&mut self) {
        self.playlist.clear();
        self.selected_playlist_index = 0;
        // Don't reset current track or playback state when clearing playlist
    }
}
