use super::*;

/// Handles state transitions and ensures they are valid
pub trait StateTransition {
    /// Validates and performs a state transition
    fn transition(&mut self, from: &AppState, to: &AppState) -> Result<(), &'static str>;
}

/// Implementation of state transitions for AppState
pub struct AppStateTransition;

impl StateTransition for AppStateTransition {
    fn transition(&mut self, from: &AppState, to: &AppState) -> Result<(), &'static str> {
        // Validate player state transitions
        match (&from.player.playback_state, &to.player.playback_state) {
            (PlaybackState::Playing, PlaybackState::Paused) => Ok(()),
            (PlaybackState::Playing, PlaybackState::Stopped) => Ok(()),
            (PlaybackState::Paused, PlaybackState::Playing) => Ok(()),
            (PlaybackState::Paused, PlaybackState::Stopped) => Ok(()),
            (PlaybackState::Stopped, PlaybackState::Playing) => {
                if to.player.current_track.is_some() {
                    Ok(())
                } else {
                    Err("Cannot play without a track loaded")
                }
            }
            (PlaybackState::Stopped, PlaybackState::Paused) => {
                Err("Cannot pause when stopped")
            }
            _ => Ok(()) // Same state transitions are allowed
        }?;

        // Validate seek state transitions
        match (&from.player.seek_state, &to.player.seek_state) {
            (SeekState::FastForward, SeekState::Normal) => Ok(()),
            (SeekState::Rewind, SeekState::Normal) => Ok(()),
            (SeekState::Normal, SeekState::FastForward) => {
                if matches!(to.player.playback_state, PlaybackState::Playing) {
                    Ok(())
                } else {
                    Err("Can only fast forward during playback")
                }
            }
            (SeekState::Normal, SeekState::Rewind) => {
                if matches!(to.player.playback_state, PlaybackState::Playing) {
                    Ok(())
                } else {
                    Err("Can only rewind during playback")
                }
            }
            _ => Ok(()) // Same state transitions are allowed
        }?;

        // Validate playlist state transitions
        if let (Some(from_idx), Some(to_idx)) = (from.playlist.selected_index, to.playlist.selected_index) {
            if to_idx >= to.playlist.tracks.len() {
                return Err("Selected track index out of bounds");
            }
            if from_idx != to_idx && matches!(to.player.playback_state, PlaybackState::Playing) {
                // Track change during playback is allowed
                Ok(())
            } else {
                Ok(())
            }
        } else {
            Ok(())
        }
    }
}
