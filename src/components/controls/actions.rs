use crate::events::{Action, PlayerAction};
use super::Controls;

pub fn update(controls: &mut Controls, action: Action) -> Option<Action> {
    match action {
        Action::Player(player_action) => handle_player_action(controls, player_action),
        Action::Refresh => Some(Action::Refresh),
        _ => None,
    }
}

fn handle_player_action(controls: &mut Controls, action: PlayerAction) -> Option<Action> {
    match action {
        PlayerAction::Play => {
            controls.is_playing = true;
            controls.is_seeking_forward = false;
            controls.is_seeking_backward = false;
            Some(Action::Refresh)
        }
        PlayerAction::Pause => {
            controls.is_playing = false;
            Some(Action::Refresh)
        }
        PlayerAction::Stop => {
            controls.is_playing = false;
            controls.is_seeking_forward = false;
            controls.is_seeking_backward = false;
            controls.is_recording = false;
            Some(Action::Refresh)
        }
        PlayerAction::FastForward => {
            controls.is_seeking_forward = true;
            controls.is_seeking_backward = false;
            Some(Action::Refresh)
        }
        PlayerAction::Rewind => {
            controls.is_seeking_backward = true;
            controls.is_seeking_forward = false;
            Some(Action::Refresh)
        }
        PlayerAction::Record => {
            controls.is_recording = !controls.is_recording;
            Some(Action::Refresh)
        }
        _ => None,
    }
}
