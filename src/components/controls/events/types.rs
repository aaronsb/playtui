use super::super::Section;
use crate::events::Action;
use crate::events::PlayerAction;

pub(crate) fn get_action_for_button(section: Section, button: usize) -> Option<Action> {
    match section {
        Section::Controls => match button {
            0 => Some(Action::Player(PlayerAction::Record)),
            1 => Some(Action::Player(PlayerAction::Play)),
            2 => Some(Action::Player(PlayerAction::Rewind)),
            3 => Some(Action::Player(PlayerAction::FastForward)),
            4 => Some(Action::Player(PlayerAction::Stop)),
            5 => Some(Action::Player(PlayerAction::Pause)),
            6 => Some(Action::Player(PlayerAction::LoadTrack(String::new()))), // Next
            7 => Some(Action::Player(PlayerAction::LoadTrack(String::new()))), // Previous
            _ => None,
        },
        Section::Volume => None, // TODO: Implement volume control
    }
}
