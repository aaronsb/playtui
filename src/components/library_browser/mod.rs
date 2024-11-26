mod state;
mod view;
mod events;

use ratatui::prelude::*;
use crate::events::{Event, Action, EventHandler, EventResult};
use crate::components::Component;
use crate::theme::Theme;
use state::LibraryBrowserState;

#[derive(Clone)]
pub struct LibraryBrowser {
    state: LibraryBrowserState,
}

impl Component for LibraryBrowser {
    fn new() -> Self {
        Self {
            state: LibraryBrowserState::new(),
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, focused: bool, theme: &Theme) {
        view::render(&self.state, frame, area, focused, theme);
    }

    fn update(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::NavigateUp | Action::NavigateDown | 
            Action::NavigateRight | Action::NavigateLeft |
            Action::Select | Action::Back => {
                // Convert navigation actions to key events
                events::process_event(&mut self.state, &Event::Key(match action {
                    Action::NavigateUp => crate::events::KeyEvent::Up,
                    Action::NavigateDown => crate::events::KeyEvent::Down,
                    Action::NavigateRight | Action::Select => crate::events::KeyEvent::Enter,
                    Action::NavigateLeft | Action::Back => crate::events::KeyEvent::Left,
                    _ => unreachable!(),
                }))
            }
            _ => None,
        }
    }

    fn focused(&self) -> bool {
        self.state.focused()
    }

    fn set_focused(&mut self, focused: bool) {
        self.state.set_focused(focused);
    }

    fn handle_event(&mut self, event: Event) -> Option<Action> {
        events::process_event(&mut self.state, &event)
    }
}

impl EventHandler for LibraryBrowser {
    fn can_handle(&self, event: &Event) -> bool {
        events::can_handle_event(&self.state, event)
    }

    fn handle_event(&mut self, event: &Event) -> EventResult<Option<Action>> {
        events::handle_event(&mut self.state, event)
    }
}
