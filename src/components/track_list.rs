use ratatui::{
    prelude::*,
    widgets::{List, ListItem},
};
use super::{Component, ComponentState, create_block};
use crate::events::{Event, Action, KeyEvent};
use crate::theme::Theme;

#[derive(Clone)]
pub struct TrackList {
    state: ComponentState,
    tracks: Vec<String>,
    selected_index: Option<usize>,
}

impl Component for TrackList {
    fn new() -> Self {
        Self {
            state: ComponentState::default(),
            tracks: Vec::new(),
            selected_index: None,
        }
    }

    fn render(&self, frame: &mut Frame, area: Rect, focused: bool, theme: &Theme) {
        let block = create_block("Track List", focused, theme);
        
        // Create a list of tracks
        let entries: Vec<ListItem> = self.tracks
            .iter()
            .enumerate()
            .map(|(i, track)| {
                let style = if Some(i) == self.selected_index {
                    theme.get_style("list_selected")
                } else {
                    theme.get_style("list_item")
                };
                
                ListItem::new(format!("🎵 {}", track))
                    .style(style)
            })
            .collect();

        let list = List::new(entries)
            .block(block)
            .highlight_style(theme.get_style("list_selected"));

        frame.render_widget(list, area);
    }

    fn update(&mut self, action: Action) -> Option<Action> {
        match action {
            Action::NavigateUp => {
                self.selected_index = Some(self.selected_index
                    .map(|i| if i > 0 { i - 1 } else { 0 })
                    .unwrap_or(0));
                Some(Action::Refresh)
            }
            Action::NavigateDown => {
                let max_index = self.tracks.len().saturating_sub(1);
                self.selected_index = Some(self.selected_index
                    .map(|i| if i < max_index { i + 1 } else { max_index })
                    .unwrap_or(0));
                Some(Action::Refresh)
            }
            Action::Select => {
                if let Some(index) = self.selected_index {
                    if index < self.tracks.len() {
                        Some(Action::Playlist(crate::events::PlaylistAction::SelectTrack(index)))
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
            Action::Back => {
                self.selected_index = None;
                Some(Action::Refresh)
            }
            _ => None,
        }
    }

    fn focused(&self) -> bool {
        self.state.focused
    }

    fn set_focused(&mut self, focused: bool) {
        self.state.focused = focused;
    }

    fn handle_event(&mut self, event: Event) -> Option<Action> {
        match event {
            Event::Key(key_event) => match key_event {
                KeyEvent::Up => Some(Action::NavigateUp),
                KeyEvent::Down => Some(Action::NavigateDown),
                KeyEvent::Enter => Some(Action::Select),
                KeyEvent::Esc => Some(Action::Back),
                _ => None,
            },
            _ => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup_track_list() -> TrackList {
        let mut track_list = TrackList::new();
        track_list.tracks = vec![
            "Track 1".to_string(),
            "Track 2".to_string(),
            "Track 3".to_string(),
        ];
        track_list
    }

    #[test]
    fn test_track_list_navigation() {
        let mut track_list = setup_track_list();
        
        // Initial state
        assert_eq!(track_list.selected_index, None);

        // Navigate down
        track_list.update(Action::NavigateDown);
        assert_eq!(track_list.selected_index, Some(0));

        // Navigate down again
        track_list.update(Action::NavigateDown);
        assert_eq!(track_list.selected_index, Some(1));

        // Navigate up
        track_list.update(Action::NavigateUp);
        assert_eq!(track_list.selected_index, Some(0));

        // Navigate up at top (should stay at 0)
        track_list.update(Action::NavigateUp);
        assert_eq!(track_list.selected_index, Some(0));
    }

    #[test]
    fn test_track_list_bounds() {
        let mut track_list = setup_track_list();
        
        // Navigate past end
        for _ in 0..5 {
            track_list.update(Action::NavigateDown);
        }
        assert_eq!(track_list.selected_index, Some(2)); // Should stop at last index

        // Navigate back up
        track_list.update(Action::NavigateUp);
        assert_eq!(track_list.selected_index, Some(1));
    }

    #[test]
    fn test_track_list_selection() {
        let mut track_list = setup_track_list();
        
        // Select without selection
        assert_eq!(track_list.update(Action::Select), None);

        // Navigate and select
        track_list.update(Action::NavigateDown);
        assert_eq!(
            track_list.update(Action::Select),
            Some(Action::Playlist(crate::events::PlaylistAction::SelectTrack(0)))
        );

        // Clear selection
        track_list.update(Action::Back);
        assert_eq!(track_list.selected_index, None);
    }

    #[test]
    fn test_track_list_event_handling() {
        let mut track_list = setup_track_list();
        
        assert_eq!(track_list.handle_event(Event::Key(KeyEvent::Up)), Some(Action::NavigateUp));
        assert_eq!(track_list.handle_event(Event::Key(KeyEvent::Down)), Some(Action::NavigateDown));
        assert_eq!(track_list.handle_event(Event::Key(KeyEvent::Enter)), Some(Action::Select));
        assert_eq!(track_list.handle_event(Event::Key(KeyEvent::Esc)), Some(Action::Back));
        assert_eq!(track_list.handle_event(Event::Key(KeyEvent::Char('a'))), None);
    }
}
