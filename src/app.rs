use std::path::PathBuf;
use crate::theme::Theme;
use crate::preferences::Preferences;

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

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Focus {
    Browser,
    Songs,
    Playlist,
}

impl Focus {
    // Define the focus order as a constant array
    const FOCUS_ORDER: [Focus; 3] = [Focus::Browser, Focus::Songs, Focus::Playlist];
    
    // Get the current focus index
    fn get_index(&self) -> usize {
        Self::FOCUS_ORDER.iter()
            .position(|f| f == self)
            .unwrap_or(0)
    }
    
    // Cycle to next focus
    fn next(&self) -> Self {
        let current_idx = self.get_index();
        let next_idx = (current_idx + 1) % Self::FOCUS_ORDER.len();
        Self::FOCUS_ORDER[next_idx]
    }
    
    // Cycle to previous focus
    fn previous(&self) -> Self {
        let current_idx = self.get_index();
        let prev_idx = if current_idx == 0 {
            Self::FOCUS_ORDER.len() - 1
        } else {
            current_idx - 1
        };
        Self::FOCUS_ORDER[prev_idx]
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum MenuPage {
    Preferences,
    Looks,
    About,
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
    pub menu_page: MenuPage,
    pub playback_position: u64,
    pub theme: Theme,
    pub selected_theme_index: usize,
    pub repeat_mode: bool,
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
            menu_page: MenuPage::Preferences,
            playback_position: 0,
            theme,
            selected_theme_index: 0,
            repeat_mode: false,
        }
    }
}

impl App {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply_preferences(&mut self, preferences: &Preferences) {
        // Apply theme
        if let Ok(theme) = Theme::load_theme(&preferences.theme) {
            self.theme = theme;
        }

        // Apply volume (convert from f32 to u8)
        self.volume = (preferences.volume * 100.0) as u8;

        // Apply repeat mode
        self.repeat_mode = preferences.repeat_mode;

        // Apply last directory if it exists
        if let Some(last_dir) = &preferences.last_directory {
            let path = PathBuf::from(last_dir);
            if path.exists() && path.is_dir() {
                self.filesystem = FileSystem::new(path);
                self.update_songs();
            }
        }
    }

    pub fn get_current_preferences(&self) -> Preferences {
        Preferences {
            theme: self.theme.theme_info.name.clone(),
            volume: self.volume as f32 / 100.0,
            repeat_mode: self.repeat_mode,
            last_directory: Some(self.filesystem.current_dir.to_string_lossy().into_owned()),
        }
    }

    pub fn toggle_menu(&mut self) {
        self.show_menu = !self.show_menu;
        if self.show_menu {
            // Reset to preferences page when opening menu
            self.menu_page = MenuPage::Preferences;
            self.selected_theme_index = 0;
        }
    }

    pub fn cycle_menu_page(&mut self) {
        if self.show_menu {
            self.menu_page = match self.menu_page {
                MenuPage::Preferences => MenuPage::Looks,
                MenuPage::Looks => MenuPage::About,
                MenuPage::About => MenuPage::Preferences,
            };
            // Reset theme selection when switching to Looks page
            if self.menu_page == MenuPage::Looks {
                self.selected_theme_index = 0;
            }
        }
    }

    pub fn apply_selected_theme(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.show_menu && self.menu_page == MenuPage::Looks {
            if let Ok(themes) = Theme::list_themes() {
                if let Some(theme_name) = themes.get(self.selected_theme_index) {
                    self.theme = Theme::load_theme(theme_name)?;
                }
            }
        }
        Ok(())
    }

    pub fn move_theme_selection(&mut self, direction: ThemeDirection) {
        if self.show_menu && self.menu_page == MenuPage::Looks {
            if let Ok(themes) = Theme::list_themes() {
                let theme_count = themes.len();
                if theme_count == 0 {
                    return;
                }

                let cols = 3;
                let _rows = (theme_count + cols - 1) / cols;
                let current_row = self.selected_theme_index / cols;
                let current_col = self.selected_theme_index % cols;

                self.selected_theme_index = match direction {
                    ThemeDirection::Up if current_row > 0 => {
                        let new_index = self.selected_theme_index - cols;
                        if new_index < theme_count { new_index } else { self.selected_theme_index }
                    },
                    ThemeDirection::Down => {
                        let new_index = self.selected_theme_index + cols;
                        if new_index < theme_count { new_index } else { self.selected_theme_index }
                    },
                    ThemeDirection::Left if current_col > 0 => {
                        self.selected_theme_index - 1
                    },
                    ThemeDirection::Right => {
                        let new_index = self.selected_theme_index + 1;
                        if new_index < theme_count && new_index % cols != 0 { new_index } else { self.selected_theme_index }
                    },
                    _ => self.selected_theme_index,
                };
            }
        }
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
            } else if self.repeat_mode {
                // If repeat mode is on and we're at the end, go back to the first track
                self.current_track_index = Some(0);
                if let Some(track) = tracks.first() {
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
            } else if self.repeat_mode {
                // If repeat mode is on and we're at the start, go to the last track
                let tracks = match self.focus {
                    Focus::Songs => &self.songs,
                    Focus::Playlist => &self.playlist,
                    _ => return,
                };
                if !tracks.is_empty() {
                    let last_index = tracks.len() - 1;
                    self.current_track_index = Some(last_index);
                    self.current_track = tracks.last().cloned();
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
        self.focus = self.focus.next();
    }

    pub fn reverse_toggle_focus(&mut self) {
        self.focus = self.focus.previous();
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

    pub fn toggle_repeat_mode(&mut self) {
        self.repeat_mode = !self.repeat_mode;
    }
}

#[derive(Debug, Clone, Copy)]
pub enum ThemeDirection {
    Up,
    Down,
    Left,
    Right,
}

