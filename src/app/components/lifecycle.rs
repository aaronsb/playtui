use crate::events::{Event, Action, SystemEvent};
use super::ComponentManager;

/// Trait for managing component lifecycle
pub trait ComponentLifecycle {
    /// Convert an action to an event for component processing
    fn convert_action_to_event(&self, action: &Action) -> Option<Event>;
}

impl ComponentLifecycle for ComponentManager {
    fn convert_action_to_event(&self, action: &Action) -> Option<Event> {
        use crate::events::{KeyEvent, PlayerAction, UIAction, AppAction};
        
        let event = match action {
            Action::Key(key_event) => Event::Key(key_event.clone()),
            Action::NavigateLeft => Event::Key(KeyEvent::Left),
            Action::NavigateRight => Event::Key(KeyEvent::Right),
            Action::NavigateUp => Event::Key(KeyEvent::Up),
            Action::NavigateDown => Event::Key(KeyEvent::Down),
            Action::Play => Event::Key(KeyEvent::Play),
            Action::Pause => Event::Key(KeyEvent::Pause),
            Action::Stop => Event::Key(KeyEvent::Stop),
            Action::NextTrack => Event::Key(KeyEvent::Next),
            Action::PreviousTrack => Event::Key(KeyEvent::Previous),
            Action::VolumeUp => Event::Key(KeyEvent::VolumeUp),
            Action::VolumeDown => Event::Key(KeyEvent::VolumeDown),
            Action::Player(player_action) => match player_action {
                PlayerAction::Play => Event::Key(KeyEvent::Play),
                PlayerAction::Pause => Event::Key(KeyEvent::Pause),
                PlayerAction::Stop => Event::Key(KeyEvent::Stop),
                PlayerAction::Record => Event::Key(KeyEvent::Record),
                PlayerAction::FastForward => Event::Key(KeyEvent::FastForward),
                PlayerAction::Rewind => Event::Key(KeyEvent::Rewind),
                PlayerAction::StopEject => Event::Key(KeyEvent::Stop),
                PlayerAction::SetVolume(_) | PlayerAction::LoadTrack(_) => Event::System(SystemEvent::TrackLoaded),
            },
            Action::UI(ui_action) => match ui_action {
                UIAction::Focus(direction) => Event::Key(KeyEvent::Focus(*direction)),
                UIAction::UpdateTheme(_) | UIAction::Resize { .. } => Event::System(SystemEvent::TrackLoaded),
            },
            Action::Playlist(_) | Action::Metadata(_) => Event::System(SystemEvent::TrackLoaded),
            Action::App(app_action) => match *app_action {
                AppAction::Error(ref msg) => Event::System(SystemEvent::Error(msg.clone())),
                AppAction::Quit => Event::Key(KeyEvent::Escape),
                AppAction::Cancel => Event::Key(KeyEvent::Escape),
                AppAction::NoOp => return None,
            },
            Action::Select => Event::Key(KeyEvent::Enter),
            Action::Back => Event::Key(KeyEvent::Escape),
            Action::Refresh => return None,
            Action::SetVolume(_) => Event::System(SystemEvent::TrackLoaded),
        };
        Some(event)
    }
}
