use crate::events::{Action, PlaylistAction};
use super::Playlist;

pub(super) fn update(playlist: &mut Playlist, action: Action) -> Option<Action> {
    match action {
        Action::Playlist(PlaylistAction::AddTrack(path)) => {
            playlist.tracks.push(path);
            // Select first track if none selected
            if playlist.list_state.selected().is_none() && !playlist.tracks.is_empty() {
                playlist.list_state.select(Some(0));
            }
            None
        }
        Action::Playlist(PlaylistAction::RemoveTrack(index)) => {
            if index < playlist.tracks.len() {
                playlist.tracks.remove(index);
                // Adjust selection if necessary
                if let Some(selected) = playlist.list_state.selected() {
                    if selected >= playlist.tracks.len() {
                        playlist.list_state.select(if playlist.tracks.is_empty() {
                            None
                        } else {
                            Some(playlist.tracks.len() - 1)
                        });
                    }
                }
            }
            None
        }
        Action::Playlist(PlaylistAction::SelectTrack(index)) => {
            if index < playlist.tracks.len() {
                playlist.list_state.select(Some(index));
                if let Some(track) = playlist.tracks.get(index) {
                    Some(Action::Player(crate::events::PlayerAction::LoadTrack(
                        track.clone(),
                    )))
                } else {
                    None
                }
            } else {
                None
            }
        }
        Action::Playlist(PlaylistAction::Clear) => {
            playlist.tracks.clear();
            playlist.list_state.select(None);
            playlist.scroll_offset = 0;
            None
        }
        _ => None,
    }
}

pub(super) fn select_next(playlist: &mut Playlist) -> Option<Action> {
    if playlist.tracks.is_empty() {
        return None;
    }

    let next_index = match playlist.list_state.selected() {
        Some(i) if i + 1 < playlist.tracks.len() => i + 1,
        Some(_) => 0, // Wrap around to start
        None => 0,
    };

    Some(Action::Playlist(PlaylistAction::SelectTrack(next_index)))
}

pub(super) fn select_previous(playlist: &mut Playlist) -> Option<Action> {
    if playlist.tracks.is_empty() {
        return None;
    }

    let prev_index = match playlist.list_state.selected() {
        Some(0) => playlist.tracks.len() - 1, // Wrap around to end
        Some(i) => i - 1,
        None => 0,
    };

    Some(Action::Playlist(PlaylistAction::SelectTrack(prev_index)))
}
