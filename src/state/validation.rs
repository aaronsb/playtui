use super::*;

/// Validates state consistency and prevents invalid states
pub trait StateValidator {
    /// Validates the current state
    fn validate(&self, state: &AppState) -> Result<(), &'static str>;
    
    /// Validates a specific component's state
    fn validate_component(&self, state: &AppState, component: &str) -> Result<(), &'static str>;
}

/// Implementation of state validation for AppState
pub struct AppStateValidator;

impl StateValidator for AppStateValidator {
    fn validate(&self, state: &AppState) -> Result<(), &'static str> {
        // Validate player state
        self.validate_player_state(state)?;

        // Validate playlist state
        self.validate_playlist_state(state)?;

        // Validate UI state
        self.validate_ui_state(state)?;

        // Validate metadata state
        self.validate_metadata_state(state)?;

        Ok(())
    }

    fn validate_component(&self, state: &AppState, component: &str) -> Result<(), &'static str> {
        match component {
            "player" => self.validate_player_state(state).map(|_| ()),
            "playlist" => self.validate_playlist_state(state),
            "ui" => self.validate_ui_state(state),
            "metadata" => self.validate_metadata_state(state),
            _ => Err("Unknown component"),
        }
    }
}

impl AppStateValidator {
    fn validate_player_state(&self, state: &AppState) -> Result<PlaybackState, &'static str> {
        // First check volume regardless of playback state
        if state.player.volume > 100 {
            return Err("Volume exceeds maximum");
        }

        match state.player.playback_state {
            PlaybackState::Playing => {
                // Additional playing state validation
                self.validate_playback_requirements(state)?;
                Ok(PlaybackState::Playing)
            }
            PlaybackState::Paused => {
                if state.player.current_track.is_none() {
                    return Err("Paused state without track");
                }
                Ok(PlaybackState::Paused)
            }
            PlaybackState::Stopped => Ok(PlaybackState::Stopped),
        }
    }

    fn validate_playback_requirements(&self, state: &AppState) -> Result<(), &'static str> {
        // Check if we have a track loaded
        if state.player.current_track.is_none() {
            return Err("No track loaded");
        }

        // Check if the track exists in playlist
        if let Some(track) = &state.player.current_track {
            if !state.playlist.tracks.contains(track) {
                return Err("Current track not in playlist");
            }
        }

        Ok(())
    }

    fn validate_playlist_state(&self, state: &AppState) -> Result<(), &'static str> {
        if let Some(index) = state.playlist.selected_index {
            if index >= state.playlist.tracks.len() {
                return Err("Selected index out of bounds");
            }
        }
        Ok(())
    }

    fn validate_ui_state(&self, state: &AppState) -> Result<(), &'static str> {
        // Validate window size
        if state.ui.window_size.0 == 0 || state.ui.window_size.1 == 0 {
            return Err("Invalid window size");
        }

        // Validate focused component
        match state.ui.focused_component.as_str() {
            "library_browser" | "track_list" | "track_details" |
            "current_track_info" | "playback_status" | "controls" |
            "volume_control" => Ok(()),
            _ => Err("Invalid focused component"),
        }
    }

    fn validate_metadata_state(&self, state: &AppState) -> Result<(), &'static str> {
        if let Some(current_track) = &state.player.current_track {
            if state.metadata.current_metadata.is_none() &&
               !state.metadata.metadata_cache.contains_key(current_track) {
                return Err("Missing metadata for current track");
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_player_state_validation() {
        let mut state = AppState::default();
        let validator = AppStateValidator;

        // Test invalid volume
        state.player.volume = 101;
        assert!(validator.validate_component(&state, "player").is_err());

        // Test valid stopped state
        state.player.volume = 50;
        state.player.playback_state = PlaybackState::Stopped;
        assert!(validator.validate_component(&state, "player").is_ok());

        // Test invalid paused state
        state.player.playback_state = PlaybackState::Paused;
        assert!(validator.validate_component(&state, "player").is_err());
    }

    #[test]
    fn test_playlist_state_validation() {
        let mut state = AppState::default();
        let validator = AppStateValidator;

        // Test empty playlist
        assert!(validator.validate_component(&state, "playlist").is_ok());

        // Test invalid selected index
        state.playlist.selected_index = Some(0);
        assert!(validator.validate_component(&state, "playlist").is_err());

        // Test valid selected index
        state.playlist.tracks.push("test.mp3".to_string());
        assert!(validator.validate_component(&state, "playlist").is_ok());
    }
}
